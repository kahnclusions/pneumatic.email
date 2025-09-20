use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Uuid;

use crate::db::Db;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub description: String,
    pub server_url: String,
    pub name: String,
    pub username: String,
    pub mailbox_state: Option<String>,
    pub thread_state: Option<String>,
    pub identity_state: Option<String>,

    pub email_state: Option<String>,
    pub email_query_state: Option<String>,
    pub email_query_last_id: Option<String>,
    pub email_query_loaded: bool,

    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccountEmail {
    pub account_id: Uuid,
    pub email_id: String,
}

#[derive(Clone, Debug)]
pub struct AccountRepo {
    db: Db,
}

impl AccountRepo {
    #[tracing::instrument]
    pub fn new(db: &Db) -> Self {
        Self { db: db.clone() }
    }

    #[tracing::instrument]
    pub async fn list_accounts(&self) -> anyhow::Result<Vec<Account>> {
        let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts")
            .fetch_all(&self.db)
            .await?;
        Ok(accounts)
    }

    #[tracing::instrument]
    pub async fn get_account(&self, id: &Uuid) -> anyhow::Result<Account> {
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = ?1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(account)
    }

    #[tracing::instrument]
    pub async fn find_account(
        &self,
        server_url: &str,
        username: &str,
    ) -> anyhow::Result<Option<Account>> {
        let account = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE username = ?1 AND server_url = ?2",
        )
        .bind(username)
        .bind(server_url)
        .fetch_optional(&self.db)
        .await?;
        Ok(account)
    }

    #[tracing::instrument(skip(access_token, refresh_token))]
    pub async fn create_account(
        &self,
        username: String,
        description: String,
        server_url: String,
        account_name: String,
        access_token: String,
        refresh_token: Option<String>,
        expires: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Account> {
        let id = Uuid::now_v7();
        sqlx::query("INSERT INTO accounts (id, description, server_url, username, name, access_token, refresh_token, expires) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")
          .bind(id.clone())
          .bind(description)
          .bind(server_url)
          .bind(username)
          .bind(account_name)
          .bind(access_token)
          .bind(refresh_token)
          .bind(expires.map(|dt| dt.to_rfc3339()))
          .execute(&self.db)
          .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument(skip(access_token, refresh_token))]
    pub async fn update_auth(
        &self,
        id: Uuid,
        access_token: String,
        refresh_token: Option<String>,
        expires: Option<DateTime<Utc>>,
    ) -> anyhow::Result<Account> {
        sqlx::query(
            "UPDATE accounts SET access_token = ?2, refresh_token = ?3, expires = ?4 WHERE id = ?1",
        )
        .bind(id.clone())
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires.map(|dt| dt.to_rfc3339()))
        .execute(&self.db)
        .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument]
    pub async fn update_mailbox_state(
        &self,
        id: &Uuid,
        mailbox_state: Option<String>,
    ) -> anyhow::Result<Account> {
        sqlx::query("UPDATE accounts SET mailbox_state = ?2 WHERE id = ?1")
            .bind(id.clone())
            .bind(mailbox_state)
            .execute(&self.db)
            .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument]
    pub async fn update_email_query(
        &self,
        id: &Uuid,
        email_query_last_id: String,
    ) -> anyhow::Result<Account> {
        sqlx::query("UPDATE accounts SET email_query_last_id = ?2 WHERE id = ?1")
            .bind(id.clone())
            .bind(email_query_last_id)
            .execute(&self.db)
            .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument]
    pub async fn update_email_query_loaded(
        &self,
        id: &Uuid,
        email_query_loaded: bool,
    ) -> anyhow::Result<Account> {
        sqlx::query("UPDATE accounts SET email_query_loaded = ?2 WHERE id = ?1")
            .bind(id.clone())
            .bind(email_query_loaded)
            .execute(&self.db)
            .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument]
    pub async fn update_email_state(
        &self,
        id: &Uuid,
        email_state: Option<String>,
    ) -> anyhow::Result<Account> {
        sqlx::query("UPDATE accounts SET email_state = ?2 WHERE id = ?1")
            .bind(id.clone())
            .bind(email_state)
            .execute(&self.db)
            .await?;

        Ok(self.get_account(&id).await?)
    }

    #[tracing::instrument]
    pub async fn insert_account_email(
        &self,
        account_id: &Uuid,
        email_id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query("INSERT OR IGNORE INTO account_email (account_id, email_id) VALUES (?1, ?2")
            .bind(account_id)
            .bind(email_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn delete_account_email(
        &self,
        account_id: &Uuid,
        email_id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM account_email WHERE account_id = ?1 AND email_id = ?2")
            .bind(account_id)
            .bind(email_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(access_token, refresh_token))]
    pub async fn upsert_account(
        &self,
        server_url: &str,
        access_token: String,
        refresh_token: Option<String>,
        expires: Option<DateTime<Utc>>,
        username: &str,
        account_name: &str,
    ) -> anyhow::Result<Account> {
        let account = self.find_account(server_url, username).await?;

        if let Some(account) = account {
            tracing::debug!("Account already exists, updating...");
            // If this account already exists, update with new auth info.
            self.update_auth(account.id, access_token, refresh_token, expires)
                .await
        } else {
            tracing::debug!("No existing account, creating...");
            // New account, insert into DB.
            self.create_account(
                username.to_owned(),
                "".to_string(),
                server_url.to_owned(),
                account_name.to_owned(),
                access_token,
                refresh_token,
                expires,
            )
            .await
        }
    }
}
