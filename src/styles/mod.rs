use iced::{theme::Palette, Theme};

use self::system::system_theme_is_dark;

pub mod application;
pub mod button;
pub mod container;
pub mod dropdown;
pub mod system;
pub mod text_input;

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTheme;
mod colors_and_themes {
    use iced::Background;

    use super::get_theme_for_main_window;

    pub fn get_background() -> Background {
        Background::Color(get_theme_for_main_window().palette().background)
    }
}

pub const SIZE_1: f32 = 4.;
pub const SIZE_2: f32 = 8.;
pub const SIZE_3: f32 = 12.;
pub const SIZE_4: f32 = 16.;
pub const SIZE_5: f32 = 20.;

pub fn get_theme_for_main_window() -> Theme {
    if system_theme_is_dark() {
        iced::Theme::CatppuccinMocha
    } else {
        iced::Theme::TokyoNightLight
    }
}

pub fn get_palette_for_main_window() -> Palette {
    get_theme_for_main_window().palette()
}
