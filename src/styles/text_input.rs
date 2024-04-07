use iced::{
    border::Radius, widget::text_input, Border, Color,
};

use super::{
    colors_and_themes, get_palette_for_main_window,
    CustomTheme,
};

impl text_input::StyleSheet for CustomTheme {
    type Style = iced::Theme;

    fn active(
        &self,
        style: &Self::Style,
    ) -> text_input::Appearance {
        get_text_input_appearance(style.palette().primary)
    }

    fn focused(
        &self,
        _style: &Self::Style,
    ) -> text_input::Appearance {
        let palette = get_palette_for_main_window();

        get_text_input_appearance(palette.primary)
    }

    fn placeholder_color(
        &self,
        style: &Self::Style,
    ) -> Color {
        let [r, g, b, a] =
            style.palette().text.into_linear();
        Color::from_linear_rgba(r, g, b, a / 3.)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        style.palette().text
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        let [r, g, b, a] =
            style.palette().text.into_linear();
        Color::from_linear_rgba(r, g, b, a / 5.)
    }

    fn selection_color(
        &self,
        style: &Self::Style,
    ) -> Color {
        style.selection_color(
            &iced::theme::TextInput::Default,
        )
    }

    fn hovered(
        &self,
        _style: &Self::Style,
    ) -> text_input::Appearance {
        let palette = get_palette_for_main_window();

        get_text_input_appearance(palette.primary)
    }

    fn disabled(
        &self,
        style: &Self::Style,
    ) -> text_input::Appearance {
        style.disabled(&iced::theme::TextInput::Default)
    }
}

pub fn get_text_input_style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(CustomTheme))
}

fn get_text_input_appearance(
    border_color: Color,
) -> text_input::Appearance {
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
