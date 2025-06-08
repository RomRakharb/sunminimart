use iced::widget::container::Style;
use iced::widget::{Space, button, column, container, row, text};
use iced::{Border, Element, Length, Pixels, color};

use crate::inventory;

#[derive(Clone, Debug)]
pub enum Message {
    GotoInventory,
    Exit,
}

pub fn update(state: &mut crate::State, message: crate::Message) {
    if let crate::Message::Home(message) = message {
        match message {
            Message::GotoInventory => {
                state.screen = crate::Screen::Inventory(inventory::State::default())
            }
            Message::Exit => {}
        }
    }
}

pub fn view<'a>() -> Element<'a, crate::Message> {
    column![
        Space::with_height(Length::Fill),
        row![
            Space::with_width(Length::Fill),
            container(
                button(text("คลังสินค้า").size(Pixels(30.0)))
                    .on_press(crate::Message::Home(Message::GotoInventory))
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
        let mut state = crate::State::default();
        update(&mut state, crate::Message::Home(Message::GotoInventory));
        assert_eq!(
            state.screen,
            crate::Screen::Inventory(inventory::State::default())
        );
    }
}
