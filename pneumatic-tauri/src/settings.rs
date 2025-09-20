use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

use crate::state::AppState;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PneumaticSettings {
    pub theme_mode: ThemeMode,
}

#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> anyhow_tauri::TAResult<PneumaticSettings> {
    let theme_mode: Option<ThemeMode> =
        serde_json::from_value(state.store.get("theme_mode").unwrap_or_default())
            .map_err(|err| anyhow!(err))?;

    Ok(PneumaticSettings {
        theme_mode: theme_mode.unwrap_or(ThemeMode::System),
    })
}
