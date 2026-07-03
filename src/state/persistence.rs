use crate::state::models::{DayLog, UserSettings};

const LOGS_KEY: &str = "nutritrack_logs";
const SETTINGS_KEY: &str = "nutritrack_settings";

fn storage() -> Option<web_sys::Storage> {
    let window = web_sys::window()?;
    window.local_storage().ok()?
}

pub fn load_logs() -> Vec<DayLog> {
    let store = storage();
    let json = store
        .as_ref()
        .and_then(|s| s.get_item(LOGS_KEY).ok())
        .flatten();
    json.and_then(|j| serde_json::from_str(&j).ok())
        .unwrap_or_default()
}

pub fn save_logs(logs: &[DayLog]) {
    if let Some(store) = storage() {
        if let Ok(json) = serde_json::to_string(logs) {
            let _ = store.set_item(LOGS_KEY, &json);
        }
    }
}

pub fn load_settings() -> UserSettings {
    let store = storage();
    let json = store
        .as_ref()
        .and_then(|s| s.get_item(SETTINGS_KEY).ok())
        .flatten();
    json.and_then(|j| serde_json::from_str(&j).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &UserSettings) {
    if let Some(store) = storage() {
        if let Ok(json) = serde_json::to_string(settings) {
            let _ = store.set_item(SETTINGS_KEY, &json);
        }
    }
}
