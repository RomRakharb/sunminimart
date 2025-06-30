use iced::alignment::Horizontal;
use iced::keyboard::key;
use iced::widget::{
    button, column, container, horizontal_space, keyed_column, row, scrollable, text, text_input,
    vertical_space,
};
use iced::{Color, Element, Length, Pixels, Subscription, Task, color, keyboard};
use reqwest;
use serde_json;

use crate::custom;
use shared::Item;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct State {
    pub all_items: Vec<Item>,
    pub filtered_items: Vec<Item>,
    pub current_item: Item,
    pub search: String,
    pub position: usize,
    pub search_focus: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Back,
    OnSearchChange(String),
    Refresh,
    FetchItems,
    ItemsFetched(Vec<Item>),
    ChangePosition(key::Named),
    PositionChanged(key::Named, bool),
    OnSearchSubmit,
    IsSearchFocus(bool),
}

pub fn update(state: &mut crate::State, message: crate::Message) -> Task<Message> {
    let mut tasks =
        vec![text_input::is_focused(text_input::Id::new("search")).map(Message::IsSearchFocus)];

    if let crate::Message::Inventory(message) = message {
        match message {
            Message::Back => {
                state.screen = crate::Screen::Home;
            }
            Message::OnSearchChange(search) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.search = search.clone();
                    state.position = 0;
                    state.filtered_items = state
                        .all_items
                        .clone()
                        .into_iter()
                        .filter(|item| {
                            item.barcode.contains(&search) || item.name.contains(&search)
                        })
                        .collect();
                }
            }
            Message::OnSearchSubmit => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    if !state.filtered_items.is_empty() {
                        state.current_item = state.filtered_items[state.position].clone();
                    } else {
                        state.current_item = Item::default();
                    }
                }
            }
            Message::Refresh => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.filtered_items = state.all_items.clone();
                    state.current_item = Item::default();
                    state.search = String::new();
                }
            }
            Message::FetchItems => tasks.push(Task::perform(
                fetch_items(state.setting.url.clone()),
                |items| Message::ItemsFetched(items),
            )),
            Message::ItemsFetched(items) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.all_items = items.clone();
                    state.filtered_items = items;
                }
            }
            Message::ChangePosition(action) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    match action {
                        key::Named::ArrowDown => {
                            if state.position < state.filtered_items.len() - 1 {
                                state.position += 1
                            }
                        }
                        key::Named::ArrowUp => {
                            if state.position > 0 {
                                state.position -= 1
                            }
                        }
                        _ => {}
                    }
                }
            }
            Message::PositionChanged(action, is_focus) => {}
            Message::IsSearchFocus(value) => {
                if let crate::Screen::Inventory(state) = &mut state.screen {
                    state.search_focus = value;
                }
            }
        }
    }
    Task::batch(tasks)
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
    output_items
}

pub fn view(state: &State) -> Element<crate::Message> {
    column![
        vertical_space(),
        custom::title("คลังสินค้า"),
        vertical_space(),
        row![
            horizontal_space(),
            column![
                row![
                    text("ค้นหา: "),
                    text_input("", &state.search)
                        .id(text_input::Id::new("search"))
                        .on_input(|input| {
                            crate::Message::Inventory(Message::OnSearchChange(input))
                        })
                        .on_submit(crate::Message::Inventory(Message::OnSearchSubmit)),
                    button("refresh").on_press(crate::Message::Inventory(Message::Refresh))
                ]
                .spacing(Pixels(10.0)),
                scrollable(keyed_column(state.filtered_items.iter().enumerate().map(
                    |(i, item)| {
                        (
                            i,
                            container(row![
                                text(item.barcode.clone())
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center),
                                text(item.name.clone())
                                    .shaping(text::Shaping::Advanced)
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center)
                            ])
                            .style(move |_| {
                                if i == state.position {
                                    container::Style {
                                        background: Some(iced::Background::Color(color!(0x4169e1))),
                                        text_color: Some(Color::WHITE),
                                        ..Default::default()
                                    }
                                } else {
                                    container::Style::default()
                                }
                            })
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
                custom::button("แก้ไขข้อมูลสินค้า").padding(20),
                custom::button("ลบสินค้า").padding(20),
                custom::button("เพิ่มรายการสินค้า").padding(20),
                custom::button("เพิ่มจำนวนสินค้า").padding(20),
                custom::button("เพิ่มรายการสินค้า").padding(20),
            ]
            .width(Length::FillPortion(2))
            .align_x(Horizontal::Center)
            .spacing(Pixels(20.0)),
            horizontal_space(),
            column![
                row![
                    text("รหัสสินค้า: ").width(Length::Fill),
                    text_input("", &state.current_item.barcode)
                        .id(text_input::Id::new("barcode"))
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("ชื่อ: ")
                        .shaping(text::Shaping::Advanced)
                        .width(Length::Fill),
                    text_input("", &state.current_item.name)
                        .id(text_input::Id::new("name"))
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("ต้นทุน: ").width(Length::Fill),
                    text_input("", &state.current_item.cost.to_string())
                        .id(text_input::Id::new("cost"))
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("ราคา: ").width(Length::Fill),
                    text_input("", &state.current_item.price.to_string())
                        .id(text_input::Id::new("price"))
                        .width(Length::FillPortion(4))
                ],
                row![
                    text("จำนวน: ").width(Length::Fill),
                    text_input("", &state.current_item.quantity.to_string())
                        .id(text_input::Id::new("quantity"))
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

pub(crate) fn subscription(state: &State) -> Subscription<crate::Message> {
    if state.search_focus {
        keyboard::on_key_press(|keyboard, _| match keyboard {
            keyboard::Key::Named(key::Named::Escape) => {
                Some(crate::Message::Inventory(Message::Back))
            }
            keyboard::Key::Named(key::Named::ArrowDown) => Some(crate::Message::Inventory(
                Message::ChangePosition(key::Named::ArrowDown),
            )),
            keyboard::Key::Named(key::Named::ArrowUp) => Some(crate::Message::Inventory(
                Message::ChangePosition(key::Named::ArrowUp),
            )),
            _ => None,
        })
    } else {
        keyboard::on_key_press(|keyboard, _| match keyboard {
            keyboard::Key::Named(key::Named::Escape) => {
                Some(crate::Message::Inventory(Message::Back))
            }
            _ => None,
        })
    }
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

    fn sample_items() -> Vec<Item> {
        vec![
            Item {
                barcode: "0".to_string(),
                name: "a".to_string(),
                ..Default::default()
            },
            Item {
                barcode: "1".to_string(),
                name: "a".to_string(),
                ..Default::default()
            },
            Item {
                barcode: "2".to_string(),
                name: "b".to_string(),
                ..Default::default()
            },
        ]
    }

    #[test]
    fn back() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }

    #[test]
    fn items_fetched() {
        let items = sample_items();
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
    fn on_search_change() {
        let items = sample_items();

        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(
            items.clone(),
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.all_items, state.filtered_items);
            assert_eq!(state.position, 1);
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "1".to_string(),
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.filtered_items, vec![items[1].clone()]);
            assert_eq!(state.position, 0);
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "a".to_string(),
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(
                state.filtered_items,
                vec![items[0].clone(), items[1].clone()]
            );
        }

        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "b".to_string(),
        )));
        if let crate::Screen::Inventory(state) = state.screen {
            assert_eq!(state.filtered_items, vec![items[2].clone()]);
        }
    }

    #[test]
    fn refresh() {
        let mut state = init_state();
        let items = sample_items();
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

    #[test]
    fn change_position() {
        let mut state = init_state();
        let items = sample_items();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.position, 0);
        }

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.position, 1);
        }

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowUp,
        )));
        if let crate::Screen::Inventory(ref state) = state.screen {
            assert_eq!(state.position, 0);
        }

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        if let crate::Screen::Inventory(state) = state.screen {
            assert_eq!(state.position, 2);
        }
    }

    #[test]
    fn on_search_submit() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(
            sample_items(),
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowUp,
        )));

        // Select as normal
        let _ = state.update(crate::Message::Inventory(Message::OnSearchSubmit));
        if let crate::Screen::Inventory(local_state) = &state.screen {
            assert_eq!(
                local_state.current_item,
                local_state.filtered_items[local_state.position]
            );
        } else {
            panic!()
        }

        // Select when empty
        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "aa".to_string(),
        )));
        let _ = state.update(crate::Message::Inventory(Message::OnSearchSubmit));
        if let crate::Screen::Inventory(local_state) = &state.screen {
            assert_eq!(local_state.current_item, Item::default());
        } else {
            panic!()
        }
    }

    // #[test]
    // fn on_enter_edit() {
    //     let mut state = init_state();
    //     let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(
    //         sample_items(),
    //     )));
    //     let _ = state.update(crate::Message::Inventory(Message::OnSearchSubmit));

    //     // let _ = state.update(crate::Message::Inventory(Message::OnEnterEdit))
    // }
}
