use std::str::FromStr;

use anyhow::bail;
use pneumatic_jmap::oauth::{
    exchange_code_for_token, lookup_oauth_server, start_authentication, OAuthChallenge,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter, EventTarget, Manager};
use tauri_plugin_opener::OpenerExt;

use crate::state::AppState;

#[tauri::command]
pub async fn open_url(app: tauri::AppHandle, href: &str) -> Result<(), String> {
    app.opener()
        .open_url(href.to_string(), None::<&str>)
        .map_err(|err| {
            tracing::error!("{err:?}");
            err.to_string()
        })?;
    Ok(())
}

#[tauri::command]
pub async fn oauth_start(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    email: &str,
) -> Result<String, String> {
    let db = &state.db;

    println!("got an auth request for {email}");

    let (server_url, oauth_server) = lookup_oauth_server(email).await.map_err(|err| {
        println!("Error: {:?}", err);
        "Failed to lookup oauth server"
    })?;

    let challenge = start_authentication(server_url.clone(), &oauth_server, email)
        .await
        .map_err(|err| {
            println!("Error: {:?}", err);
            "Internal server error".to_string()
        })?;

    state
        .challenge_repo
        .insert_challenge(
            email,
            &server_url,
            &challenge.csrf_token,
            &challenge.verifier,
            &challenge.auth_url,
        )
        .await
        .map_err(|err| {
            tracing::error!("{err:?}");
            println!("{err:?}");
            err.to_string()
        })?;

    use tauri_plugin_opener::OpenerExt;
    app_handle
        .opener()
        .open_url(challenge.auth_url, None::<&str>)
        .map_err(|err| {
            tracing::error!("{err:?}");
            println!("{err:?}");
            err.to_string()
        })?;
    Ok("opening".to_string())
}

pub async fn oauth_callback(handle: AppHandle, url: String) -> anyhow::Result<()> {
    let state = handle.state::<AppState>();
    let url = url::Url::from_str(url.as_str())?;
    let auth_code = url.query_pairs().find_map(|(k, v)| {
        if k == "code" {
            Some(v.to_string())
        } else {
            None
        }
    });
    let csrf_token = url.query_pairs().find_map(|(k, v)| {
        if k == "state" {
            Some(v.to_string())
        } else {
            None
        }
    });
    let Some(auth_code) = auth_code else {
        bail!("No `code` was included in callback url")
    };
    let Some(csrf_token) = csrf_token else {
        bail!("No `state` was included in callback url")
    };
    let challenge = state.challenge_repo.get_challenge(&csrf_token).await?;
    let server_url = challenge.server_url.clone();
    let challenge = OAuthChallenge {
        id: challenge.id,
        email: challenge.email,
        server_url: challenge.server_url,
        csrf_token: challenge.csrf_token,
        verifier: challenge.verifier,
        token_url: challenge.token_url,
    };
    let jmap_access = exchange_code_for_token(&auth_code, challenge).await?;

    let account = state
        .account_repo
        .upsert_account(
            &server_url,
            jmap_access.access_token,
            jmap_access.refresh_token,
            jmap_access.expires,
            &jmap_access.username,
            &jmap_access.account_name,
        )
        .await?;

    // let credentials = Credentials::bearer(account.access_token.clone());
    // let client = Client::new()
    //     .credentials(credentials)
    //     .connect(&account.server_url)
    //     .await?;
    //
    // if account.mailbox_state.is_none() {
    //     jmap::cold_boot(&state.db, &client, &account).await?;
    // }

    handle.emit_to(EventTarget::any(), "oauth-success", account.id)?;
    Ok(())
}
