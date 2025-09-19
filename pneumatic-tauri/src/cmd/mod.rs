use pneumatic_data::account::Account;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountDTO {
    pub id: Uuid,
    pub description: String,
    pub name: String,
    pub username: String,
    pub server_url: String,
}

impl From<Account> for AccountDTO {
  fn from(value: Account) -> Self {
    Self {
        id: value.id,
        description: value.description,
        server_url: value.server_url,
        name: value.name,
        username: value.username,
    }
  }
}

#[tauri::command]
pub async fn fetch_accounts(
    state: tauri::State<'_, AppState>,
) -> anyhow_tauri::TAResult<Vec<AccountDTO>> {
    let account_list = state.account_repo.list_accounts().await?;
    let account_list: Vec<AccountDTO> =
        account_list.into_iter().map(|acc| acc.into()).collect();

    // for account in account_list.iter_mut() {
    //     let mailboxes = list_mailboxes(&state.db, &account.id).await?;
    //     let mailboxes: Vec<_> = mailboxes.into_iter().map(|mb| mb.into()).collect();
    //     account.mailboxes = mailboxes;
    // }

    Ok(account_list)
}
