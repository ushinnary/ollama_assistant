use iced::{color, theme::Palette};

use self::system::system_theme_is_dark;

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

pub const PADDING_SIZE: u16 = 8;

pub fn get_palette_for_main_window() -> Palette {
    if system_theme_is_dark() {
        // Oxacarbon without background
        Palette {
            background: color!(0x000000, 0.0),
            ..iced::Theme::Oxocarbon.palette()
        }
    } else {
        // TokyoNightLight
        Palette {
            background: color!(0x000000, 0.0),
            ..iced::Theme::TokyoNightLight.palette()
        }
    }
}
