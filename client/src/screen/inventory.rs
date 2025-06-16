use iced::alignment::Horizontal;
use iced::widget::{
    column, horizontal_space, keyed_column, row, scrollable, text, text_input, vertical_space,
};
use iced::{Element, Length, Pixels};

use crate::custom_widget;
use shared::Item;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    pub items: Vec<Item>,
    pub item: Item,
    pub search: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

pub fn update(state: &mut crate::State, message: crate::Message) {
    if let crate::Message::Inventory(message) = message {
        match message {
            Message::Back => state.screen = crate::Screen::Home,
        }
    }
}

pub fn view<'a>(state: &State) -> Element<'a, crate::Message> {
    column![
        custom_widget::title("คลังสินค้า"),
        vertical_space(),
        row![
            horizontal_space(),
            column![
                row![text("ค้นหา: "), text_input("", &state.search)],
                scrollable(keyed_column((1..=100).map(|i| {
                    (
                        i,
                        row![
                            text(i).width(Length::Fill).align_x(Horizontal::Center),
                            custom_widget::advanced_text(format!("สินค้าราคา {} บาท", i))
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                        ]
                        .into(),
                    )
                })))
                .width(Length::Fill)
            ]
            .width(Length::FillPortion(6))
            .spacing(Pixels(10.0)),
            horizontal_space(),
            column![
                row![
                    text("รหัสสินค้า: ").width(Length::Fill),
                    text_input("", &state.item.barcode).width(Length::FillPortion(4))
                ],
                row![
                    custom_widget::advanced_text("ชื่อ: ").width(Length::Fill),
                    text_input("", &state.item.name).width(Length::FillPortion(4))
                ],
                row![
                    text("ต้นทุน: ").width(Length::Fill),
                    text_input("", &state.item.cost.to_string()).width(Length::FillPortion(4))
                ],
                row![
                    text("ราคา: ").width(Length::Fill),
                    text_input("", &state.item.price.to_string()).width(Length::FillPortion(4))
                ],
                row![
                    text("จำนวน: ").width(Length::Fill),
                    text_input("", &state.item.quantity.to_string()).width(Length::FillPortion(4))
                ]
            ]
            .width(Length::FillPortion(6))
            .spacing(Pixels(10.0)),
            horizontal_space(),
        ]
        .height(Length::FillPortion(12)),
        vertical_space()
    ]
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn back() {
        let mut state = crate::State {
            screen: crate::Screen::Inventory(Box::default()),
            ..Default::default()
        };
        state.update(crate::Message::Inventory(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }
}
