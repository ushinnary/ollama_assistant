use iced::{
    alignment::{Horizontal, Vertical},
    widget::{button, container, svg, text, text_input, Row},
    Element, Length,
};

use crate::{
    styles::{button::get_btn_transparent_style, CustomTheme, PADDING_SIZE},
    MainMessage,
};

pub fn top_bar<'a>(settings_icon: svg::Handle) -> impl Into<Element<'a, MainMessage>> {
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
                    svg(settings_icon)
                        .width(Length::Fixed(18.))
                        .height(Length::Fixed(18.)),
                )
                .style(get_btn_transparent_style()),
            ),
        )
        .padding([0, 0, PADDING_SIZE, 0])
}

pub fn search_bar<'a>(text: &str) -> impl Into<Element<'a, MainMessage>> {
    container(
        text_input("AI Message", text)
            .padding(PADDING_SIZE)
            .size(20)
            .style(iced::theme::TextInput::Custom(Box::new(CustomTheme)))
            .on_input(MainMessage::UpdateInput)
            .on_submit(MainMessage::SendToAI),
    )
    .width(Length::Fill)
    .center_x()
}
