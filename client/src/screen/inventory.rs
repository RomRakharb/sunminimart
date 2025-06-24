use iced::alignment::Horizontal;
use iced::keyboard::key::Named;
use iced::widget::{
    button, column, horizontal_space, keyed_column, row, scrollable, text, text_input,
    vertical_space,
};
use iced::{Element, Length, Pixels, Subscription, Task, keyboard};
use reqwest;
use serde_json;

use crate::custom_widget;
use shared::Item;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    pub all_items: Vec<Item>,
    pub filtered_items: Vec<Item>,
    pub current_item: Item,
    pub search: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    OnSearchChange(String),
    Refresh,
    FetchItems,
    ItemsFetched(Vec<shared::Item>),
}

pub fn update(state: &mut crate::State, message: crate::Message) -> Task<crate::Message> {
    if let crate::Message::Inventory(message) = message {
        match message {
            Message::Back => {
                state.screen = crate::Screen::Home;
                Task::none()
            }
            Message::OnSearchChange(search) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.search = search.clone();
                    state.filtered_items = state
                        .all_items
                        .clone()
                        .into_iter()
                        .filter(|item| {
                            item.barcode.contains(&search) || item.name.contains(&search)
                        })
                        .collect();
                }
                Task::none()
            }
            Message::Refresh => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.filtered_items = state.all_items.clone();
                    state.current_item = Item::default();
                    state.search = String::new();
                }
                Task::none()
            }
            Message::FetchItems => Task::perform(fetch_items(state.setting.url.clone()), |items| {
                crate::Message::Inventory(Message::ItemsFetched(items))
            }),
            Message::ItemsFetched(items) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.all_items = items.clone();
                    state.filtered_items = items;
                }
                Task::none()
            }
        }
    } else {
        Task::none()
    }
}

pub(super) async fn fetch_items(url: String) -> Vec<Item> {
    let mut output_items = Vec::new();
    match reqwest::get(format!("{url}/items")).await {
        Err(e) => eprintln!("reqwest error: {e}"),
        Ok(items) => match items.text().await {
            Err(e) => eprintln!("test error: {e}"),
            Ok(items) => {
                output_items = serde_json::from_str(&items).unwrap();
            }
        },
    }
    println!("fetch_item at {}", url);
    output_items
}

pub fn view<'a>(state: &State) -> Element<'a, crate::Message> {
    column![
        custom_widget::title("คลังสินค้า"),
        vertical_space(),
        row![
            horizontal_space(),
            column![
                row![
                    text("ค้นหา: "),
                    text_input("", &state.search).on_input(|input| {
                        crate::Message::Inventory(Message::OnSearchChange(input))
                    }),
                    button("refresh").on_press(crate::Message::Inventory(Message::Refresh))
                ],
                scrollable(keyed_column(state.filtered_items.iter().enumerate().map(
                    |(i, item)| {
                        (
                            i,
                            row![
                                text(item.barcode.clone())
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center),
                                custom_widget::advanced_text(item.name.clone())
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center)
                            ]
                            .into(),
                        )
                    }
                )))
                .width(Length::Fill)
            ]
            .width(Length::FillPortion(6))
            .spacing(Pixels(10.0)),
            horizontal_space(),
            column![
                row![
                    text("รหัสสินค้า: ").width(Length::Fill),
                    text_input("", &state.current_item.barcode).width(Length::FillPortion(4))
                ],
                row![
                    custom_widget::advanced_text("ชื่อ: ").width(Length::Fill),
                    text_input("", &state.current_item.name).width(Length::FillPortion(4))
                ],
                row![
                    text("ต้นทุน: ").width(Length::Fill),
                    text_input("", &state.current_item.cost.to_string())
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("ราคา: ").width(Length::Fill),
                    text_input("", &state.current_item.price.to_string())
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("จำนวน: ").width(Length::Fill),
                    text_input("", &state.current_item.quantity.to_string())
                        .width(Length::FillPortion(4))
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

pub(crate) fn subscription(_state: &State) -> Subscription<crate::Message> {
    keyboard::on_key_release(|keyboard, _| match keyboard {
        keyboard::Key::Named(Named::Escape) => Some(crate::Message::Inventory(Message::Back)),
        _ => None,
    })
}

#[cfg(test)]
mod test {
    use rust_decimal::prelude::Zero;

    use super::*;

    fn init_state() -> crate::State {
        crate::State {
            screen: crate::Screen::Inventory(Box::default()),
            ..Default::default()
        }
    }

    #[test]
    fn back() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }

    // #[test]
    // fn fetch_items() {
    //     let mut state = init_state();
    //     let task: Task<crate::Message> =
    //         state.update(crate::Message::Inventory(Message::FetchItems));
    //     assert_ne!(task, Task::none());
    // }

    #[test]
    fn items_fetched() {
        let items = vec![
            shared::Item {
                barcode: "0".to_string(),
                ..Default::default()
            },
            shared::Item {
                barcode: "1".to_string(),
                ..Default::default()
            },
            shared::Item {
                barcode: "2".to_string(),
                ..Default::default()
            },
        ];
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));
        if let crate::Screen::Inventory(state) = state.screen {
            assert_eq!(state.all_items.len(), 3);
            assert_eq!(state.all_items[0].barcode, "0".to_string());
            assert_eq!(state.all_items[1].barcode, "1".to_string());
            assert_eq!(state.all_items[2].barcode, "2".to_string());
        }
    }

    #[test]
    fn on_search_changed() {
        let item_0 = shared::Item {
            barcode: "0".to_string(),
            name: "a".to_string(),
            ..Default::default()
        };
        let item_1 = shared::Item {
            barcode: "1".to_string(),
            name: "a".to_string(),
            ..Default::default()
        };
        let item_2 = shared::Item {
            barcode: "2".to_string(),
            name: "b".to_string(),
            ..Default::default()
        };
        let items = vec![item_0.clone(), item_1.clone(), item_2.clone()];

        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.all_items, state.filtered_items);
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "1".to_string(),
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.filtered_items, vec![item_1.clone()]);
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "a".to_string(),
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.filtered_items, vec![item_0, item_1]);
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "b".to_string(),
        )));
        if let crate::Screen::Inventory(state) = state.screen {
            assert_eq!(state.filtered_items, vec![item_2]);
        }
    }

    #[test]
    fn refresh() {
        let mut state = init_state();
        let items = vec![
            shared::Item {
                barcode: "0".to_string(),
                ..Default::default()
            },
            shared::Item {
                barcode: "1".to_string(),
                ..Default::default()
            },
            shared::Item {
                barcode: "2".to_string(),
                ..Default::default()
            },
        ];
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));

        if let crate::Screen::Inventory(ref mut state) = state.screen {
            state.current_item = Item {
                barcode: "test".to_string(),
                name: "test".to_string(),
                cost: rust_decimal::dec!(5.6),
                price: rust_decimal::dec!(8.2),
                quantity: 34,
                ..Default::default()
            };
        }
        let _ = state.update(crate::Message::Inventory(Message::Refresh));
        if let crate::Screen::Inventory(state) = state.screen {
            assert_eq!(state.all_items, state.filtered_items);
            assert!(state.current_item.barcode.is_empty());
            assert!(state.current_item.name.is_empty());
            assert!(state.current_item.cost.is_zero());
            assert!(state.current_item.price.is_zero());
            assert!(state.current_item.quantity.is_zero());
            assert!(state.current_item.image.is_none());
            assert!(state.current_item.expire_date.is_empty());
            assert!(state.current_item.bulk_item.is_empty());
        } else {
            panic!();
        }
    }
}
