use chrono::Datelike;
use iced::alignment::{Horizontal, Vertical};
use iced::keyboard::key;
use iced::widget::text::LineHeight;
use iced::widget::{
    button, column, container, horizontal_space, row, text, text_input, vertical_space,
};
use iced::{Color, Element, Length, Pixels, Subscription, Task, color, keyboard};
use reqwest;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json;

use crate::custom;
use shared::Item;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct State {
    pub all_items: Vec<Item>,
    pub filtered_items: Vec<Item>,
    pub current_item: Item,
    pub position: usize,
    pub search: Search,
    pub mode: Mode,
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Search {
    value: String,
    focus: bool,
}

#[derive(Default, Debug, PartialEq)]
pub(crate) enum Mode {
    #[default]
    Search,
    Edit,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    IsSearchFocus(bool),
    OnSearchChange(String),
    OnSearchSubmit,
    Refresh,

    FetchItems,
    ItemsFetched(Vec<Item>),

    ChangePosition(key::Named),
    PositionChanged(key::Named, bool),

    EnterEditMode,

    OnNameChange(String),
    OnCostChange(String),
    OnPriceChange(String),
    OnQuantityChange(String),
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
                modify(state, |state| {
                    state.search.value = search.to_string();
                    state.position = 0;
                    state.filtered_items = state
                        .all_items
                        .clone()
                        .into_iter()
                        .filter(|item| {
                            item.barcode.contains(&search) || item.name.contains(&search)
                        })
                        .collect();
                });
            }
            Message::OnSearchSubmit => {
                modify(state, |state| match state.filtered_items.is_empty() {
                    true => state.current_item = Item::default(),
                    false => state.current_item = state.filtered_items[state.position].clone(),
                });
            }
            Message::OnNameChange(name) => {
                modify(state, |state| {
                    state.current_item.name = name;
                });
            }
            Message::OnCostChange(cost) => {
                modify(state, |state| {
                    if let Ok(cost) = cost.parse::<f32>() {
                        if let Some(cost) = Decimal::from_f32(cost) {
                            state.current_item.cost = cost;
                        }
                    }
                });
            }
            Message::OnPriceChange(price) => {
                modify(state, |state| {
                    if let Ok(price) = price.parse::<f32>() {
                        if let Some(price) = Decimal::from_f32(price) {
                            state.current_item.price = price;
                        }
                    }
                });
            }
            Message::OnQuantityChange(quantity) => {
                modify(state, |state| {
                    if let Ok(quantity) = quantity.parse::<i32>() {
                        state.current_item.quantity = quantity;
                    }
                });
            }
            Message::Refresh => {
                modify(state, |state| {
                    state.filtered_items = state.all_items.clone();
                    state.current_item = Item::default();
                    state.search.value = String::new();
                    state.position = 0;
                });
                tasks.push(text_input::focus(text_input::Id::new("search")))
            }
            Message::FetchItems => tasks.push(Task::perform(
                fetch_items(state.setting.url.clone()),
                Message::ItemsFetched,
            )),
            Message::ItemsFetched(items) => {
                modify(state, |state| {
                    state.all_items = items.clone();
                    state.filtered_items = items;
                });
            }
            Message::ChangePosition(action) => {
                modify(state, |state| match action {
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
                });
            }
            Message::PositionChanged(_, _) => {}
            Message::IsSearchFocus(value) => {
                modify(state, |state| {
                    state.search.focus = value;
                });
            }
            Message::EnterEditMode => {
                modify(state, |state| {
                    state.mode = Mode::Edit;
                });
            }
        }
    } else {
        panic!("Message error in inventory");
    }

    Task::batch(tasks)
}

fn modify<F>(state: &mut crate::State, f: F)
where
    F: FnOnce(&mut State),
{
    if let crate::Screen::Inventory(ref mut state) = state.screen {
        f(state);
    } else {
        panic!("Screen error in inventory");
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
                    text("ค้นหา: ")
                        .line_height(LineHeight::Relative(2.0))
                        .align_y(Vertical::Center),
                    text_input("", &state.search.value)
                        .id(text_input::Id::new("search"))
                        .on_input(|input| {
                            crate::Message::Inventory(Message::OnSearchChange(input))
                        })
                        .on_submit(crate::Message::Inventory(Message::OnSearchSubmit)),
                    button("refresh").on_press(crate::Message::Inventory(Message::Refresh))
                ]
                .spacing(Pixels(10.0)),
                custom::list(state.filtered_items.clone(), |i, item| {
                    container(row![
                        text(item.barcode.clone())
                            .line_height(LineHeight::Relative(2.0))
                            .width(Length::Fill)
                            .align_x(Horizontal::Center),
                        text(item.name.clone())
                            .line_height(LineHeight::Relative(2.0))
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
                    .into()
                })
                .width(Length::Fill)
            ]
            .width(Length::FillPortion(6))
            .spacing(Pixels(10.0)),
            horizontal_space(),
            column![
                custom::button(
                    "แก้ไขข้อมูลสินค้า",
                    crate::Message::Inventory(Message::EnterEditMode)
                )
                .padding(20),
                // custom::button("ลบสินค้า").padding(20),
                // custom::button("เพิ่มรายการสินค้า").padding(20),
                // custom::button("เพิ่มจำนวนสินค้า").padding(20),
                // custom::button("เพิ่มรายการสินค้า").padding(20),
            ]
            .width(Length::FillPortion(2))
            .align_x(Horizontal::Center)
            .spacing(Pixels(20.0)),
            horizontal_space(),
            column![
                custom::labeled_text_input(
                    "รหัสสินค้า: ",
                    &state.current_item.barcode,
                    Some("barcode"),
                    None
                ),
                custom::labeled_text_input(
                    "ชื่อ: ",
                    &state.current_item.name,
                    Some("name"),
                    Some(text::Shaping::Advanced)
                ),
                custom::labeled_text_input(
                    "ต้นทุน: ",
                    &state.current_item.cost.to_string(),
                    Some("cost"),
                    None
                ),
                custom::labeled_text_input(
                    "ราคา: ",
                    &state.current_item.price.to_string(),
                    Some("price"),
                    None
                ),
                custom::labeled_text_input(
                    "จำนวน: ",
                    &state.current_item.quantity.to_string(),
                    Some("quantity"),
                    None
                ),
                custom::list(state.current_item.expire_date.clone(), |i, expire_date| {
                    row![text(format!("{i}: ")), text_input("", ""), button("x")].into()
                }),
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
    if state.search.focus {
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
    use super::*;

    fn init_state() -> crate::State {
        crate::State {
            screen: crate::Screen::Inventory(*Box::default()),
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

    fn test<F>(state: &crate::State, f: F)
    where
        F: FnOnce(&State),
    {
        if let crate::Screen::Inventory(state) = &state.screen {
            f(state);
        } else {
            panic!("Screen error in inventory");
        }
    }

    #[test]
    fn back() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }

    #[test]
    fn fetch_item() {
        let items = sample_items();
        let mut state = init_state();

        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));
        test(&state, |state| {
            assert_eq!(state.all_items.len(), 3);
            assert_eq!(state.all_items[0].barcode, "0".to_string());
            assert_eq!(state.all_items[1].barcode, "1".to_string());
            assert_eq!(state.all_items[2].barcode, "2".to_string());
        });
    }

    #[test]
    fn search() {
        let items = sample_items();
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(
            items.clone(),
        )));

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        test(&state, |state| {
            assert_eq!(state.filtered_items, state.all_items);
            assert_eq!(state.position, 1);
        });

        // Search from barcode and reset position on search
        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "1".to_string(),
        )));
        test(&state, |state| {
            assert_eq!(state.filtered_items, vec![items[1].clone()]);
            assert_eq!(state.position, 0);
        });

        // Search from Name
        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "a".to_string(),
        )));
        test(&state, |state| {
            assert_eq!(
                state.filtered_items,
                vec![items[0].clone(), items[1].clone()]
            );
        });

        // Submit as normal
        let _ = state.update(crate::Message::Inventory(Message::OnSearchSubmit));
        test(&state, |state| {
            assert_eq!(state.current_item, state.filtered_items[state.position]);
        });

        // Submit when empty
        let _ = state.update(crate::Message::Inventory(Message::OnSearchChange(
            "aa".to_string(),
        )));
        let _ = state.update(crate::Message::Inventory(Message::OnSearchSubmit));
        test(&state, |state| {
            assert_eq!(state.current_item, Item::default());
        });
    }

    #[test]
    fn refresh() {
        let mut state = init_state();
        let items = sample_items();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));

        modify(&mut state, |state| {
            state.current_item = Item {
                barcode: "test".to_string(),
                name: "test".to_string(),
                cost: rust_decimal::dec!(5.6),
                price: rust_decimal::dec!(8.2),
                quantity: 34,
                ..Default::default()
            };
        });

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        let _ = state.update(crate::Message::Inventory(Message::Refresh));
        test(&state, |state| {
            // TODO: Find a way to test whether search is focused
            // assert!(state.search.focus);
            assert_eq!(state.all_items, state.filtered_items);
            assert_eq!(state.position, 0);
            assert!(state.current_item.barcode.is_empty());
            assert!(state.current_item.name.is_empty());
            assert!(state.current_item.cost.is_zero());
            assert!(state.current_item.price.is_zero());
            assert_eq!(state.current_item.quantity, 0);
            assert!(state.current_item.image.is_none());
            assert!(state.current_item.expire_date.is_empty());
            assert!(state.current_item.bulk_item.is_empty());
        });
    }

    #[test]
    fn change_position() {
        let mut state = init_state();
        let items = sample_items();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(items)));
        test(&state, |state| {
            assert_eq!(state.position, 0);
        });

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        test(&state, |state| {
            assert_eq!(state.position, 1);
        });

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowUp,
        )));
        test(&state, |state| {
            assert_eq!(state.position, 0);
        });

        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        let _ = state.update(crate::Message::Inventory(Message::ChangePosition(
            key::Named::ArrowDown,
        )));
        test(&state, |state| {
            assert_eq!(state.position, 2);
        });
    }

    #[test]
    fn enter_mode() {
        let mut state = init_state();
        test(&state, |state| {
            assert_eq!(state.mode, Mode::Search);
        });

        let _ = state.update(crate::Message::Inventory(Message::EnterEditMode));
        test(&state, |state| {
            assert_eq!(state.mode, Mode::Edit);
        });

        // TODO: more mode
    }

    #[test]
    fn edit() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Inventory(Message::ItemsFetched(
            sample_items(),
        )));
        let _ = state.update(crate::Message::Inventory(Message::EnterEditMode));

        // Edit name
        let _ = state.update(crate::Message::Inventory(Message::OnNameChange(
            "ขนม sunminimart".to_string(),
        )));
        test(&state, |state| {
            assert_eq!(state.current_item.name, "ขนม sunminimart".to_string());
        });

        // Edit cost
        let _ = state.update(crate::Message::Inventory(Message::OnCostChange(
            "16.5".to_string(),
        )));
        test(&state, |state| {
            assert_eq!(state.current_item.cost, Decimal::new(165, 1));
        });

        // Edit price
        let _ = state.update(crate::Message::Inventory(Message::OnPriceChange(
            "21".to_string(),
        )));
        test(&state, |state| {
            assert_eq!(state.current_item.price, Decimal::new(21, 0))
        });

        // Edit quantity
        let _ = state.update(crate::Message::Inventory(Message::OnQuantityChange(
            10.to_string(),
        )));
        test(&state, |state| {
            assert_eq!(state.current_item.quantity, 10);
        })
    }
}
