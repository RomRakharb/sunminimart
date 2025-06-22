use iced::widget::container::Style;
use iced::widget::text::Shaping;
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Alignment, Border, Element, Length, Pixels, color};

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
    container(row![
        horizontal_space().width(Length::Fill),
        container(row![
            column![
                button(
                    text("คลังสินค้า")
                        .size(Pixels(30.0))
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                )
                .padding(20)
                .on_press(crate::Message::Home(Message::GotoInventory)),
                button(
                    text("ตั้งค่า")
                        .shaping(Shaping::Advanced)
                        .size(Pixels(30.0))
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                )
                .padding(20)
                .on_press(crate::Message::Home(Message::GotoSetting)),
            ]
            .spacing(Pixels(20.0)),
        ])
        .padding(50)
        .style(|_| Style {
            border: Border {
                color: color!(0x000000),
                width: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .width(Length::Fill),
        horizontal_space().width(Length::Fill),
    ])
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
