use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    Container, Text, button, column, container, keyed_column, row, scrollable, text, text_input,
};
use iced::{Element, Length, Pixels};

use shared::Item;

pub(crate) fn title<'a>(value: impl text::IntoFragment<'a>) -> Container<'a, Message> {
    container(text(value).size(Pixels(30.0)))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
}
