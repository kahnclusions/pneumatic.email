use std::sync::Arc;

use pneumatic_data::{account::AccountRepo, oauth2_challenge::OAuth2ChallengeRepo};
use tauri::Wry;
use tauri_plugin_store::Store;


#[derive(Clone)]
pub struct AppState {
    pub db: pneumatic_data::db::Db,
    pub account_repo: AccountRepo,
    pub challenge_repo: OAuth2ChallengeRepo,
    pub store: Arc<Store<Wry>>,
}
