use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, checkbox, combo_box, container,
        horizontal_rule, svg, text, text_input,
        vertical_space, Column, Row, Scrollable,
    },
    Element, Length,
};

use crate::{
    styles::{
        button::get_btn_transparent_style,
        text_input::get_text_input_style, PADDING_SIZE,
    },
    AppState, MainMessage,
};

use super::RouterView;

pub fn top_bar<'a>(
    icon: svg::Handle,
    view_on_click: RouterView,
    api_is_live: bool,
) -> impl Into<Element<'a, MainMessage>> {
    Row::new()
        .push(
            text("AI Overlay")
                .size(20)
                .width(Length::Shrink)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Left),
        )
        .push(
            container(
                checkbox(
                    if api_is_live {
                        "API Works"
                    } else {
                        "API is OFF"
                    },
                    api_is_live,
                )
                .on_toggle_maybe(if api_is_live {
                    None
                } else {
                    Some(|_| MainMessage::RunAiHealthCheck)
                })
                .spacing(4)
                .size(16),
            )
            .padding(4)
            .width(Length::FillPortion(1)),
        )
        .push(
            Row::new().push(
                button(
                    svg(icon)
                        .width(Length::Fixed(18.))
                        .height(Length::Fixed(18.)),
                )
                .on_press(MainMessage::ChangeView(
                    view_on_click,
                ))
                .style(get_btn_transparent_style()),
            ),
        )
        .padding([0, 0, PADDING_SIZE, 0])
}

pub fn search_bar<'a>(
    text: &str,
) -> impl Into<Element<'a, MainMessage>> {
    container(
        text_input("AI Message", text)
            .padding(PADDING_SIZE)
            .size(20)
            .style(get_text_input_style())
            .on_input(MainMessage::UpdateInput)
            .on_submit(MainMessage::SendToAI),
    )
    .width(Length::Fill)
    .center_x()
}

pub fn main_page_content<'a>(
    app_state: &AppState,
    user_input: &str,
    ai_response: &str,
    error: &Option<String>,
) -> impl Into<Element<'a, MainMessage>> {
    let ai_input = search_bar(user_input).into();

    match (app_state, ai_response, error) {
        (AppState::Done, response, None)
            if !response.is_empty() =>
        {
            let scroll = Scrollable::new(
                Column::new()
                    .push(vertical_space().height(4))
                    .push(
                        container(text(response))
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .padding([
                                0,
                                PADDING_SIZE,
                                0,
                                PADDING_SIZE,
                            ]),
                    )
                    .height(155),
            );

            Column::new()
                .push(ai_input)
                .push(
                    container(text("AI's response : "))
                        .padding([
                            PADDING_SIZE,
                            0,
                            PADDING_SIZE,
                            PADDING_SIZE,
                        ]),
                )
                .push(horizontal_rule(1))
                .push(scroll)
        }
        (AppState::Done, _, Some(err_msg)) => Column::new()
            .push(ai_input)
            .push(vertical_space().height(4))
            .push(
                container(text("There was an error :("))
                    .center_x(),
            )
            .push(vertical_space().height(4))
            .push(text(err_msg)),
        (AppState::Done, _, None) => {
            Column::new().push(ai_input)
        }
        (AppState::Loading, _, _) => Column::new().push(
            container(text("In progress ..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
        ),
    }
}

pub fn settings_page_content<'a>(
    models: &'a combo_box::State<String>,
    current_model: Option<&String>,
) -> impl Into<Element<'a, MainMessage>> {
    container(combo_box(
        models,
        "Select AI Model",
        current_model,
        MainMessage::UpdateConfigModel,
    ))
}
