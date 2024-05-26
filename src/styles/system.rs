use std::{env, process::Command};

enum DesktopEnvironment {
    Gnome,
    Kde,
    Unknown,
}

impl From<String> for DesktopEnvironment {
    fn from(de: String) -> Self {
        match de.as_str() {
            "GNOME" => DesktopEnvironment::Gnome,
            "KDE" => DesktopEnvironment::Kde,
            _ => DesktopEnvironment::Unknown,
        }
    }
}

/// Check if system theme is dark
pub fn system_theme_is_dark() -> bool {
    match get_current_desktop_environment() {
        DesktopEnvironment::Gnome
        | DesktopEnvironment::Kde => {
            let system_theme = Command::new("gsettings")
                .args([
                    "get",
                    "org.gnome.desktop.interface",
                    "color-scheme",
                ])
                .output()
                .expect("Failed to get color scheme");

            if let Ok(system_theme) =
                String::from_utf8(system_theme.stdout)
            {
                system_theme.contains("dark")
            } else {
                false
            }
        }
        DesktopEnvironment::Unknown => false,
    }
}

fn get_current_desktop_environment() -> DesktopEnvironment {
    match env::var("XDG_CURRENT_DESKTOP") {
        Ok(val) => val.into(),
        Err(_e) => DesktopEnvironment::Unknown,
    }
}
