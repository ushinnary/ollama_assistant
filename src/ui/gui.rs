use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, container, horizontal_rule, svg, text,
        text_input, vertical_space, Column, Row,
        Scrollable,
    },
    Element, Length,
};

use crate::{
    styles::{
        button::get_btn_transparent_style, CustomTheme,
        PADDING_SIZE,
    },
    MainMessage, State,
};

use super::RouterView;

pub fn top_bar<'a>(
    icon: svg::Handle,
    view_on_click: RouterView,
) -> impl Into<Element<'a, MainMessage>> {
    Row::new()
        .push(
            text("AI Overlay")
                .size(20)
                .width(Length::FillPortion(2))
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Left),
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
            .style(iced::theme::TextInput::Custom(
                Box::new(CustomTheme),
            ))
            .on_input(MainMessage::UpdateInput)
            .on_submit(MainMessage::SendToAI),
    )
    .width(Length::Fill)
    .center_x()
}

pub fn main_page_content<'a>(
    app_state: &State,
    user_input: &str,
    ai_response: &str,
    error: &Option<String>,
) -> impl Into<Element<'a, MainMessage>> {
    let ai_input = search_bar(user_input).into();

    match (app_state, ai_response, error) {
        (State::Done, response, None)
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
                    .height(185),
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
        (State::Done, _, Some(err_msg)) => Column::new()
            .push(ai_input)
            .push(vertical_space().height(4))
            .push(
                container(text("There was an error :("))
                    .center_x(),
            )
            .push(vertical_space().height(4))
            .push(text(err_msg)),
        (State::Done, _, None) => {
            Column::new().push(ai_input)
        }
        (State::Loading, _, _) => Column::new().push(
            container(text("In progress ..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y(),
        ),
    }
}
