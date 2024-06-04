use std::rc::Rc;

use iced::{
    border::Radius, overlay::menu, widget::pick_list,
};

use super::{colors_and_themes, CustomTheme};

impl pick_list::StyleSheet for CustomTheme {
    type Style = iced::Theme;

    fn active(
        &self,
        style: &<Self as pick_list::StyleSheet>::Style,
    ) -> pick_list::Appearance {
        pick_list::Appearance {
            background: colors_and_themes::get_background(),
            border: iced::Border {
                color: style.palette().primary,
                width: 1.0,
                radius: Radius::from(8.0),
            },
            text_color: style.palette().text,
            handle_color: style.palette().primary,
            placeholder_color: style.palette().success,
        }
    }

    fn hovered(
        &self,
        style: &<Self as pick_list::StyleSheet>::Style,
    ) -> pick_list::Appearance {
        pick_list::Appearance {
            border: iced::Border {
                color: style.palette().success,
                width: 1.0,
                radius: Radius::from(8.0),
            },
            placeholder_color: style.palette().success,
            ..self.active(style)
        }
    }
}

impl iced::overlay::menu::StyleSheet for CustomTheme {
    type Style = iced::Theme;

    fn appearance(
        &self,
        style: &Self::Style,
    ) -> menu::Appearance {
        menu::Appearance {
            text_color: style.palette().text,
            background: colors_and_themes::get_background(),
            border: iced::Border {
                color: style.palette().success,
                width: 1.0,
                radius: Radius::from(8.0),
            },
            selected_text_color: style.palette().primary,
            selected_background: iced::Background::Color(
                iced::Color::TRANSPARENT,
            ),
        }
    }
}

impl From<CustomTheme> for iced::theme::PickList {
    fn from(value: CustomTheme) -> Self {
        Self::Custom(Rc::new(value), Rc::new(value))
    }
}

pub fn get_dropdown_style() -> iced::theme::PickList {
    CustomTheme.into()
}
