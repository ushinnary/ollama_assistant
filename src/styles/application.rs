use iced::Color;

#[derive(Default)]
pub struct ApplicationStyle;

impl iced::application::StyleSheet for ApplicationStyle {
    type Style = iced::Theme;

    fn appearance(
        &self,
        style: &Self::Style,
    ) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: Color::TRANSPARENT,
            ..style.appearance(
                &iced::theme::Application::Default,
            )
        }
    }
}

pub fn get_application_styles() -> iced::theme::Application
{
    iced::theme::Application::Custom(Box::new(
        ApplicationStyle,
    ))
}
