use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{self, Button, Container, Row, container, row, text, text_input};
use iced::{Length, Pixels};

pub(crate) fn title<'a>(value: impl text::IntoFragment<'a>) -> Container<'a, Message> {
    container(text(value).size(Pixels(30.0)))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

pub(crate) fn button<'a>(
    value: impl text::IntoFragment<'a>,
    on_press: Message,
) -> Button<'a, Message> {
    widget::button(text(value).align_x(Horizontal::Center).width(Length::Fill))
        .on_press(on_press)
        .width(Length::Fill)
}

pub(crate) fn labeled_text_input<'a>(
    label: &'a str,
    value: &str,
    id: Option<&'static str>,
    shaping: Option<text::Shaping>,
) -> Row<'a, Message> {
    let mut input_label: text::Text = text(label).width(Length::Fill);
    let mut text_input: text_input::TextInput<'_, Message> =
        text_input("", value).width(Length::FillPortion(5));

    if let Some(shaping) = shaping {
        input_label = input_label.shaping(shaping);
    }
    if let Some(id) = id {
        text_input = text_input.id(text_input::Id::new(id))
    }

    row![input_label, text_input]
}
