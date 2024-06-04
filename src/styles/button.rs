use iced::{widget::button, Border, Shadow};

pub struct TransparentButton;
impl button::StyleSheet for TransparentButton {
    type Style = iced::Theme;

    fn active(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: None,
            border: Border {
                radius: 0.0.into(),
                width: 0.,
                color: style.palette().primary,
            },
            shadow: Shadow::default(),
            text_color: style.palette().text,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset
                + iced::Vector::new(0.0, 1.0),
            ..active
        }
    }

    fn pressed(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

pub fn get_btn_transparent_style() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(TransparentButton))
}

pub struct PrimaryButton;
impl button::StyleSheet for PrimaryButton {
    type Style = iced::Theme;

    fn active(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: Some(iced::Background::Color(
                style.palette().primary,
            )),
            border: Border {
                radius: 4.0.into(),
                width: 1.,
                color: style.palette().primary,
            },
            shadow: Shadow::default(),
            text_color: iced::Color::WHITE,
        }
    }

    fn hovered(
        &self,
        style: &Self::Style,
    ) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(
                style.palette().success,
            )),
            ..self.active(style)
        }
    }
}

pub fn get_btn_primary_style() -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(PrimaryButton))
}
