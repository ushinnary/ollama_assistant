use iced::{
    border::Radius,
    color,
    theme::Palette,
    widget::{container, text_input},
    Border, Color,
};

use crate::styles::system::system_theme_is_dark;

pub const PADDING_SIZE: u16 = 8;

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTheme;

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomStyle;

impl container::StyleSheet for CustomTheme {
    type Style = CustomStyle;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(16.0),
            },
            background: Some(colors_and_themes::get_background()),
            ..Default::default()
        }
    }
}

fn get_text_input_appearance(border_color: Color) -> text_input::Appearance {
    text_input::Appearance {
        background: colors_and_themes::get_background(),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        icon_color: border_color,
    }
}

impl text_input::StyleSheet for CustomTheme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        get_text_input_appearance(_style.placeholder_color(&iced::theme::TextInput::Default))
    }

    fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = get_palette_for_main_window();

        get_text_input_appearance(palette.primary)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        style.placeholder_color(&iced::theme::TextInput::Default)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.value_color(&iced::theme::TextInput::Default)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        style.disabled_color(&iced::theme::TextInput::Default)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        style.selection_color(&iced::theme::TextInput::Default)
    }

    fn hovered(&self, _style: &Self::Style) -> text_input::Appearance {
        let palette = get_palette_for_main_window();

        get_text_input_appearance(palette.primary)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        style.disabled(&iced::theme::TextInput::Default)
    }
}

mod colors_and_themes {
    use iced::{Background, Color};

    use super::system_theme_is_dark;

    pub fn get_background() -> Background {
        Background::Color(if system_theme_is_dark() {
            Color::BLACK
        } else {
            Color::WHITE
        })
    }
}

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
