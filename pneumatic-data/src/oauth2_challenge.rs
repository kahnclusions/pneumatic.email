use serde::{Deserialize, Serialize};
use sqlx::prelude::*;

use crate::db::Db;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct OAuth2Challenge {
  email: String,
  server_url: String,
  csrf_token: String,
  verifier: String,
  token_url: String
}

pub struct OAuth2ChallengeRepo {
  db: Db
}

impl OAuth2ChallengeRepo {
  pub fn new(db: &Db) -> Self {
    Self {
      db: db.clone()
    }
  }

  /// Insert an OAuth2 challenge into the database. 
  ///
  /// A challenge has two parts, a csrf_token and a verifier. The csrf_token 
  /// validates that the same client initiated and completes the authentication 
  /// flow, and the verifier is used to validate that the OAuth2 server responded 
  /// to the challenge we generated.
  pub async fn insert_challenge(
    &self,
    email: &str,
    server_url: &str,
    csrf_token: &str,
    verifier: &str,
    token_url: &str,
  ) -> anyhow::Result<OAuth2Challenge> {
    sqlx::query("INSERT INTO oauth2_challenges (email, server_url, csrf_token, verifier, token_url) VALUES (?1, ?2, ?3, ?4, ?5)")
        .bind(email)
        .bind(server_url)
        .bind(csrf_token)
        .bind(verifier)
        .bind(token_url)
        .execute(&self.db)
        .await?;

    let challenge =
        sqlx::query_as::<_, OAuth2Challenge>("SELECT * FROM oauth2_challenges WHERE csrf_token = ?1")
            .bind(csrf_token)
            .fetch_one(&self.db)
            .await?;

    Ok(challenge)
  }

  /// Get an OAuth2 challenge from the database by its csrf_token. Clients MUST
  /// possess the right csrf token in order to complete an authentication challenge.
  pub async fn get_challenge(
    &self,
    csrf_token: &str
    ) -> anyhow::Result<OAuth2Challenge> {
    let challenge =
        sqlx::query_as::<_, OAuth2Challenge>("SELECT * FROM oauth2_challenges WHERE csrf_token = ?1")
            .bind(csrf_token)
            .fetch_one(&self.db)
            .await?;

    Ok(challenge)
  }

  pub async fn delete_challenge(
      &self,
      csrf_token: &str,
  ) -> anyhow::Result<()> {
      sqlx::query("DELETE FROM oauth2_challenges WHERE csrf_token = ?1")
          .bind(csrf_token)
          .execute(&self.db)
          .await?;

      Ok(())
  }
}
