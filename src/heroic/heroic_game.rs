use std::path::Path;

use super::heroic_platform::InstallationMode;
use serde::Deserialize;
use steam_shortcuts_util::{shortcut::ShortcutOwned, Shortcut};

#[derive(Deserialize, Debug, Clone)]
pub struct HeroicGame {
    pub app_name: String,
    pub title: String,
    pub is_dlc: bool,
    pub install_path: String,
    pub executable: String,
    pub launch_parameters: String,
    #[serde(skip_deserializing)]
    pub install_mode: Option<InstallationMode>,
    #[serde(skip_deserializing)]
    pub launch_through_heroic: bool,
}

impl HeroicGame {
    pub fn is_installed(&self) -> bool {
        Path::new(&self.install_path)
            .join(&self.executable)
            .exists()
    }
}

impl From<HeroicGame> for ShortcutOwned {
    fn from(game: HeroicGame) -> Self {
        let mut owned_shortcut = if game.launch_through_heroic && game.install_mode.is_some() {
            let launch_parameter = format!("heroic://launch/{}", game.app_name);
            let (exe, parameter) = match game.install_mode.unwrap() {
                InstallationMode::FlatPak => (
                    "flatpak",
                    format!("run com.heroicgameslauncher.hgl {}", launch_parameter),
                ),
                InstallationMode::UserBin => ("heroic", launch_parameter),
            };
            Shortcut::new(
                "0",
                game.title.as_str(),
                exe,
                "",
                "",
                "",
                parameter.as_str(),
            )
            .to_owned()
        } else {
            let target_path = Path::new(&game.install_path).join(game.executable);

            #[cfg(target_family = "unix")]
            let mut target = target_path.to_string_lossy().to_string();
            #[cfg(target_family = "unix")]
            {
                if !target.starts_with('\"') && !target.ends_with('\"') {
                    target = format!("\"{}\"", target);
                }
            }

            #[cfg(target_family = "unix")]
            let mut install_path = game.install_path.to_string();
            #[cfg(target_family = "unix")]
            {
                if !install_path.starts_with('\"') && !install_path.ends_with('\"') {
                    install_path = format!("\"{}\"", install_path);
                }
            }
            #[cfg(target_os = "windows")]
            let install_path = game.install_path.to_string();

            #[cfg(target_os = "windows")]
            let target = target_path.to_string_lossy().to_string();

            let shortcut = Shortcut::new(
                "0",
                game.title.as_str(),
                &target,
                &install_path,
                &target,
                "",
                game.launch_parameters.as_str(),
            );

            shortcut.to_owned()
        };
        owned_shortcut.tags.push("Heroic".to_owned());
        owned_shortcut.tags.push("Ready TO Play".to_owned());
        owned_shortcut.tags.push("Installed".to_owned());

        owned_shortcut
    }
}
