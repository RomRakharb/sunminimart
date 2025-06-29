use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{self, Button, Container, container, text};
use iced::{Length, Pixels};

pub(crate) fn title<'a>(value: impl text::IntoFragment<'a>) -> Container<'a, Message> {
    container(text(value).size(Pixels(30.0)))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}

pub(crate) fn button<'a>(value: impl text::IntoFragment<'a>) -> Button<'a, Message> {
    widget::button(text(value).align_x(Horizontal::Center).width(Length::Fill)).width(Length::Fill)
}
