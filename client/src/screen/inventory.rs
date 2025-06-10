use chrono::NaiveDate;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    column, container, horizontal_space, keyed_column, row, scrollable, text, text_input,
    vertical_space,
};
use iced::{Element, Length, Pixels};
use rust_decimal::Decimal;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    pub items: Vec<Item>,
    pub search: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Default, Debug, PartialEq)]
pub struct Item {
    pub barcode: String,
    pub name: String,
    pub cost: Decimal,
    pub price: Decimal,
    pub amount: i32,
    pub image: Vec<u8>,
    pub expire_date: Vec<NaiveDate>,
    pub bulk_item: Vec<BulkItem>,
}

#[derive(Default, Debug, PartialEq)]
pub struct BulkItem {
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub price: Decimal,
    pub amount: i32,
    pub image: Option<Vec<u8>>,
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
        container(text("คลังสินค้า").size(Pixels(30.0)))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
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
                            text(format!("สินค้าราคา {} บาท", i))
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                        ]
                        .into(),
                    )
                })))
                .width(Length::Fill)
            ]
            .width(Length::FillPortion(6)),
            horizontal_space(),
            column![
                text("text"),
                text("text"),
                text("text"),
                text("text"),
                text("text"),
                text("text"),
            ]
            .width(Length::FillPortion(6)),
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
            screen: crate::Screen::Inventory(State::default()),
        };
        state.update(crate::Message::Inventory(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }
}
