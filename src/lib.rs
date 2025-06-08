pub mod custom_widget;
pub mod inventory;

use inventory::Inventory;
use main::MainMessage;

#[derive(Default)]
pub struct State {
    pub page: Page,
}

#[derive(Default, Debug, PartialEq)]
pub enum Page {
    #[default]
    MainPage,
    InventoryPage(Inventory),
}

#[derive(Clone, Debug)]
pub enum Message {
    MainMessage(MainMessage),
    InventoryMessage,
}

pub mod main {
    use iced::widget::container::Style;
    use iced::widget::{Space, button, column, container, row, text};
    use iced::{Border, Color, Element, Length, Pixels, color};

    use super::{Inventory, Message, Page, State};

    #[derive(Clone, Debug)]
    pub enum MainMessage {
        GotoInventory,
        Exit,
    }

    pub fn main_update(state: &mut State, message: Message) {
        if let Message::MainMessage(message) = message {
            match message {
                MainMessage::GotoInventory => {
                    state.page = Page::InventoryPage(Inventory::default())
                }
                MainMessage::Exit => {}
            }
        }
    }

    pub fn main_view<'a>() -> Element<'a, Message> {
        column![
            Space::with_height(Length::Fill),
            row![
                Space::with_width(Length::Fill),
                container(
                    button(text("คลังสินค้า").size(Pixels(30.0)))
                        .on_press(Message::MainMessage(MainMessage::GotoInventory))
                )
                .padding(50)
                .style(|_| Style {
                    border: Border {
                        color: color!(0x000000),
                        width: 1.0,
                        ..Default::default()
                    },
                    background: Some(iced::Background::Color(color!(0xFFFFCC))),
                    ..Default::default()
                }),
                Space::with_width(Length::Fill),
            ],
            Space::with_height(Length::Fill),
        ]
        .into()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn goto_inventory() {
            let mut state = State::default();
            main_update(&mut state, Message::MainMessage(MainMessage::GotoInventory));
            assert_eq!(state.page, Page::InventoryPage(Inventory::default()));
        }
    }
}
