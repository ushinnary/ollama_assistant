use iced::{
    border::Radius, widget::container, Border, Color,
};

use super::{colors_and_themes, CustomTheme};

impl container::StyleSheet for CustomTheme {
    type Style = iced::Theme;

    fn appearance(
        &self,
        _style: &Self::Style,
    ) -> container::Appearance {
        container::Appearance {
            border: Border {
                color: Color::BLACK,
                width: 1.0,
                radius: Radius::from(16.0),
            },
            background: Some(
                colors_and_themes::get_background(),
            ),
            text_color: Some(_style.palette().text),
            ..Default::default()
        }
    }
}
