use std::{env, process::Command};

enum DesktopEnvironment {
    Gnome,
    Unknown,
}

impl From<String> for DesktopEnvironment {
    fn from(de: String) -> Self {
        match de.as_str() {
            "GNOME" => DesktopEnvironment::Gnome,
            _ => DesktopEnvironment::Unknown,
        }
    }
}

/// Check if system theme is dark
pub fn system_theme_is_dark() -> bool {
    match get_current_desktop_environment() {
        DesktopEnvironment::Gnome => {
            let system_theme = Command::new("gsettings")
                .args(["get", "org.gnome.desktop.interface", "color-scheme"])
                .output()
                .expect("Failed to get color scheme");
            let system_theme = String::from_utf8(system_theme.stdout).unwrap();

            system_theme.contains("dark")
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
