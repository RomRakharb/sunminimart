use iced::widget::container::Style;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, text};
use iced::{Border, Element, Length, Pixels, color};

use crate::screen::setting;

#[derive(Clone, Debug)]
pub enum Message {
    GotoInventory,
    GotoSetting,
}

pub fn update(state: &mut crate::State, message: crate::Message) {
    if let crate::Message::Home(message) = message {
        state.screen = match message {
            Message::GotoInventory => crate::Screen::Inventory(Box::default()),
            Message::GotoSetting => crate::Screen::Setting(setting::State::default()),
        };
    }
}

pub fn view<'a>() -> Element<'a, crate::Message> {
    container(
        container(
            column![
                button(text("คลังสินค้า").size(Pixels(30.0)))
                    .on_press(crate::Message::Home(Message::GotoInventory)),
                button(text("ตั้งค่า").shaping(Shaping::Advanced).size(Pixels(30.0)))
                    .on_press(crate::Message::Home(Message::GotoSetting)),
            ]
            .spacing(Pixels(10.0)),
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
    )
    .center(Length::Fill)
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_state() -> crate::State {
        crate::State::default()
    }

    #[test]
    fn goto_inventory() {
        let mut state = init_state();
        state.update(crate::Message::Home(Message::GotoInventory));
        assert_eq!(state.screen, crate::Screen::Inventory(Box::default()));
    }

    #[test]
    fn goto_setting() {
        let mut state = init_state();
        state.update(crate::Message::Home(Message::GotoSetting));
        assert_eq!(
            state.screen,
            crate::Screen::Setting(setting::State::default())
        );
    }
}
