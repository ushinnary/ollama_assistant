use iced::{theme::Palette, Theme};

use self::system::system_theme_is_dark;

pub mod application;
pub mod button;
pub mod container;
pub mod system;
pub mod text_input;

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTheme;
mod colors_and_themes {
    use iced::Background;

    use super::get_theme_for_main_window;

    pub fn get_background() -> Background {
        Background::Color(
            get_theme_for_main_window()
                .palette()
                .background,
        )
    }
}

pub const SIZE_1: u16 = 4;
pub const SIZE_2: u16 = 8;
pub const SIZE_3: u16 = 12;
pub const SIZE_4: u16 = 16;
pub const SIZE_5: u16 = 20;

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
