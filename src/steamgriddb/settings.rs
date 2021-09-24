use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SteamGridDbSettings {
    pub enabled: bool,
    pub auth_key: Option<String>,
}

impl Default for SteamGridDbSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_key: None,
        }
    }
}
