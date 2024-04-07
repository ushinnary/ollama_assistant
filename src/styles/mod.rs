use iced::{color, theme::Palette, Theme};

use self::system::system_theme_is_dark;

pub mod application;
pub mod button;
pub mod container;
pub mod system;
pub mod text_input;

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTheme;
mod colors_and_themes {
    use iced::{Background, Color};

    use super::system::system_theme_is_dark;

    pub fn get_background() -> Background {
        Background::Color(if system_theme_is_dark() {
            Color::BLACK
        } else {
            Color::WHITE
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AppTheme {
    Light,
    Dark,
}

pub fn get_app_theme() -> AppTheme {
    if system_theme_is_dark() {
        AppTheme::Dark
    } else {
        AppTheme::Light
    }
}

pub const PADDING_SIZE: u16 = 8;

pub fn get_theme_for_main_window() -> Theme {
    if system_theme_is_dark() {
        // Oxacarbon without background
        iced::Theme::Oxocarbon
    } else {
        // TokyoNightLight
        iced::Theme::TokyoNightLight
    }
}

pub fn get_palette_for_main_window() -> Palette {
    get_theme_for_main_window().palette()
}
