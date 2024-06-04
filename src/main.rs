use config::ApplicationSettings;
use iced::keyboard::key::Named;
use iced::widget::{column, container, svg};
use iced::window::Position;
use iced::{
    executor, keyboard, window, Application, Command,
    Element, Length, Settings, Size, Subscription,
};

use styles::application::get_application_styles;
use styles::container::get_container_style;
use styles::{get_theme_for_main_window, SIZE_3};
use ui::gui::{
    main_page_content, settings_page_content, top_bar,
};
use ui::RouterView;
use update::handle_update;

mod ai;
mod config;
mod macros;
mod styles;
mod ui;
mod update;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            decorations: false,
            size: Size::new(800.0, 300.0),
            position: Position::Centered,
            resizable: false,
            transparent: true,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    UpdateInput(String),
    SendToAI,
    AIResponse(Result<String, String>),
    ChangeView(RouterView),
    AiHealthCheck(bool),
    RunAiHealthCheck,
    UpdateConfigModel(String),
    UpdateAvailableModels(Vec<String>),
    GetAvailableModels,
    ClearAiHistory,
    Exit,
}

#[derive(Debug, Clone)]
pub enum AppState {
    Loading,
    Done,
}

pub struct App {
    text: String,
    ai_response: String,
    loading: AppState,
    error: Option<String>,
    view: RouterView,
    config_settings: ApplicationSettings,
    is_ai_api_live: bool,
    settings_icon: svg::Handle,
    back_icon: svg::Handle,
    // current_theme: styles::AppTheme,
    available_models: Vec<String>,
}

impl App {
    fn new() -> Self {
        Self {
            text: "".to_string(),
            ai_response: "".to_string(),
            loading: AppState::Done,
            error: None,
            view: RouterView::Home,
            config_settings: config::load_settings(),
            is_ai_api_live: false,
            settings_icon: svg::Handle::from_memory(
                include_bytes!("../assets/settings.svg")
                    .to_vec(),
            ),
            back_icon: svg::Handle::from_memory(
                include_bytes!("../assets/back.svg")
                    .to_vec(),
            ),
            // current_theme: styles::get_app_theme(),
            available_models: vec![],
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = MainMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(
        _flags: Self::Flags,
    ) -> (Self, Command<Self::Message>) {
        (
            App::new(),
            Command::batch(vec![
                Command::perform(
                    crate::ai::get_ai_models_installed(),
                    |v| {
                        MainMessage::UpdateAvailableModels(
                            v.unwrap_or_default(),
                        )
                    },
                ),
                Command::perform(
                    crate::ai::check_ai_health(),
                    MainMessage::AiHealthCheck,
                ),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("AI Overlay")
    }

    fn update(
        &mut self,
        message: MainMessage,
    ) -> Command<MainMessage> {
        handle_update(self, message)
    }

    fn view(&self) -> Element<MainMessage> {
        let (header_icon, on_icon_click) = match &self.view
        {
            RouterView::Home => (
                self.settings_icon.clone(),
                RouterView::Settings,
            ),
            RouterView::Settings => {
                (self.back_icon.clone(), RouterView::Home)
            }
        };

        let header = top_bar(
            header_icon,
            on_icon_click,
            self.is_ai_api_live,
        )
        .into();

        let content = match self.view {
            RouterView::Home => main_page_content(
                &self.loading,
                &self.text,
                &self.ai_response,
                &self.error,
            )
            .into(),
            RouterView::Settings => settings_page_content(
                self.available_models.clone(),
                Some(self.config_settings.ai_model.clone()),
            )
            .into(),
        };

        container(column![header, content])
            .style(get_container_style())
            .height(Length::Fill)
            .width(Length::Shrink)
            .padding(SIZE_3)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| match key
            .as_ref()
        {
            keyboard::Key::Named(Named::Escape) => {
                Some(MainMessage::Exit)
            }
            keyboard::Key::Named(Named::Backspace) => Some(
                MainMessage::ChangeView(RouterView::Home),
            ),
            keyboard::Key::Named(Named::Enter) => {
                Some(MainMessage::SendToAI)
            }
            _ => None,
        })
    }

    fn theme(&self) -> Self::Theme {
        get_theme_for_main_window()
    }

    fn style(
        &self,
    ) -> <Self::Theme as iced::application::StyleSheet>::Style
    {
        get_application_styles()
    }
}
