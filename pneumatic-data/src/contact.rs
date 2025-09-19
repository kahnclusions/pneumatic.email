use anyhow::bail;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::FromRow;

use crate::db::Db;

#[derive(Debug, Serialize, Deserialize)]
pub enum ContactProperty {
    ContactEmail { value: String, tag: String },
    ContactPhone { value: String, tag: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactData {
    pub id: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub properties: Vec<ContactProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub data: ContactData,
}

pub async fn insert_contact(db: &Db, c: Contact) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO contacts (id, name) VALUES (?1, ?2)")
        .bind(&c.id)
        .bind(&serde_json::to_vec(&c.data).unwrap())
        .execute(db)
        .await?;
    Ok(())
}

pub async fn insert_contact_email(db: &Db, id: &Uuid, email: String) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO contact_email (contact_id, email) VALUES (?1, ?2)")
        .bind(&id)
        .bind(&email)
        .execute(db)
        .await?;

    Ok(())
}
