mod cmd;
mod macos;
mod oauth;
mod settings;
mod state;

use pneumatic_data::{account::AccountRepo, db::init_db, oauth2_challenge::OAuth2ChallengeRepo};
#[cfg(desktop)]
use std::path::PathBuf;
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{utils::config::BackgroundThrottlingPolicy, WebviewUrl, WebviewWindowBuilder};
#[cfg(desktop)]
use tauri::{AppHandle, Listener, Manager, Url};
#[cfg(desktop)]
use tauri_plugin_fs::FsExt;
use tauri_plugin_store::StoreExt;

use crate::{
    oauth::{oauth_start, open_url},
    settings::get_settings,
    state::AppState,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init());

    #[cfg(desktop)]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
        let _ = app
            .get_webview_window("main")
            .expect("no main window")
            .set_focus();
        // let files = get_files_from_argv(argv.clone());
        // if !files.is_empty() {
        //     allow_file_in_scopes(app, files.clone());
        // }
        // app.emit("single-instance", Payload { args: argv, cwd })
        //   .unwrap();
    }));

    #[cfg(target_os = "macos")]
    let builder = builder.plugin(macos::traffic_light::init());

    builder
        .invoke_handler(tauri::generate_handler![
            greet,
            oauth_start,
            cmd::fetch_accounts,
            get_settings,
            open_url,
            #[cfg(target_os = "macos")]
            macos::traffic_light::set_traffic_lights,
        ])
        .setup(|app| {
            let store = app.store("pneumatic.json")?;

            // Initialize the database.
            let handle = app.handle().to_owned();
            tauri::async_runtime::block_on(async move {
                let mut data_dir = handle
                    .path()
                    .app_data_dir()
                    .expect("failed to get data_dir");
                let db = init_db(&data_dir).await;
                let account_repo = AccountRepo::new(&db);
                let challenge_repo = OAuth2ChallengeRepo::new(&db);
                tracing::info!("Database initialized");
                handle.manage(AppState {
                    db,
                    store,
                    account_repo,
                    challenge_repo,
                });
            });

            // Setup deep links, primarily to handle OAuth callbacks.
            #[cfg(any(target_os = "macos", windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                let handle = app.handle().to_owned();
                app.deep_link().on_open_url(move |event| {
                    use tauri::async_runtime::spawn;

                    let url = event
                        .urls()
                        .iter()
                        .map(|url| url.to_string())
                        .into_iter()
                        .next()
                        .unwrap();
                    tracing::debug!("got some deep links: {:?}", url);
                    let handle = handle.clone();
                    spawn(async move {
                        use crate::oauth::oauth_callback;

                        let res = oauth_callback(handle, url).await;
                        if let Err(err) = res {
                            tracing::error!(
                                "Failed to handle oauth callback because: {:?}",
                                err.to_string()
                            );
                        }
                    });
                });
            }

            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .background_throttling(BackgroundThrottlingPolicy::Disabled)
                .background_color(tauri::window::Color(50, 49, 48, 255));

            #[cfg(target_os = "macos")]
            let win_builder = win_builder
                .decorations(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .title("");

            win_builder.build().unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
