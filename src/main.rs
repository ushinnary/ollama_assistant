use config::ApplicationSettings;
use iced::keyboard::key::Named;
use iced::widget::container::StyleSheet;
use iced::widget::{column, container, svg, text};
use iced::window::Position;
use iced::{
    application, executor, keyboard, window, Application,
    Command, Element, Length, Settings, Size, Subscription,
};

use ai::check_ai_health;
use styles::{get_palette_for_main_window, CustomTheme};
use ui::gui::{main_page_content, top_bar};
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
            Command::perform(
                check_ai_health(),
                MainMessage::AiHealthCheck,
            ),
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
            RouterView::Settings => {
                column![text("WIP".to_string())].into()
            }
        };

        container(column![header, content])
            .style(CustomTheme.appearance(&self.theme()))
            .height(Length::Fill)
            .width(Length::Shrink)
            .padding(12)
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
        iced::Theme::custom(
            "Transparent".to_string(),
            get_palette_for_main_window(),
        )
    }

    fn style(
        &self,
    ) -> <Self::Theme as application::StyleSheet>::Style
    {
        <Self::Theme as application::StyleSheet>::Style::default()
    }
}
