use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{Container, Text, container, text};
use iced::{Length, Pixels};

pub(crate) fn advanced_text<'a>(value: impl text::IntoFragment<'a>) -> Text<'a> {
    iced::widget::text(value).shaping(text::Shaping::Advanced)
}

pub(crate) fn title<'a>(value: impl text::IntoFragment<'a>) -> Container<'a, Message> {
    container(text(value).size(Pixels(30.0)))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
