use iced::{
    Element, Length, Pixels, Subscription,
    keyboard::{self, key::Named},
    widget::{button, container, horizontal_space, row, text_input},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

pub(crate) const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/asset/setting.json");

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub(crate) struct State {
    pub(crate) url: String,
}

impl Default for State {
    fn default() -> Self {
        read()
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    OnIPChange(String),
    Connect,
    Back,
}

pub(crate) fn update(state: &mut crate::State, message: crate::Message) {
    if let crate::Message::Setting(message) = message {
        match message {
            Message::OnIPChange(ip) => state.screen = crate::Screen::Setting(State { url: ip }),
            Message::Connect => {
                if let crate::Screen::Setting(setting) = &state.screen {
                    state.setting.url = setting.url.clone();
                    let _ = save(setting);
                } else {
                    panic!("panic at screens::setting::update, Message::Submit");
                }
            }
            Message::Back => state.screen = crate::Screen::Home,
        }
    }
}

pub(crate) fn view<'a>(state: &State) -> Element<'a, crate::Message> {
    row![
        horizontal_space().width(Length::Fill),
        container(
            row![
                text_input("", &state.url)
                    .on_input(|input| { crate::Message::Setting(Message::OnIPChange(input)) }),
                button("บันทึก").on_press(crate::Message::Setting(Message::Connect))
            ]
            .spacing(Pixels(10.0)),
        )
        .width(Length::FillPortion(5))
        .center(Length::Fill),
        horizontal_space().width(Length::Fill),
    ]
    .into()
}

pub(crate) fn subscription(_state: &State) -> Subscription<crate::Message> {
    keyboard::on_key_release(|keyboard, _| match keyboard {
        keyboard::Key::Named(Named::Escape) => Some(crate::Message::Setting(Message::Back)),
        _ => None,
    })
}

pub(crate) fn read() -> State {
    let mut default_setting = State {
        url: "".to_string(),
    };

    if let Ok(data) = fs::read_to_string(PATH) {
        if let Ok(setting) = serde_json::from_str(&data) {
            default_setting = setting
        }
    }

    default_setting
}

fn save(setting: &State) -> std::io::Result<()> {
    let json_data = serde_json::to_string_pretty(&setting).unwrap();
    let mut file = fs::File::create(PATH)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_state() -> crate::State {
        crate::State {
            screen: crate::Screen::Setting(State::default()),
            ..Default::default()
        }
    }

    #[test]
    fn back() {
        let mut state = init_state();
        let _ = state.update(crate::Message::Setting(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }

    #[test]
    fn change_ip() {
        let ip = "192.168.1.45:3000".to_string();
        let original_ip = read();

        let mut state = init_state();

        let _ = state.update(crate::Message::Setting(Message::OnIPChange(ip.clone())));
        if let crate::Screen::Setting(state) = &state.screen {
            assert_eq!(state.url, ip);
        } else {
            panic!("test: change_ip");
        }

        let _ = state.update(crate::Message::Setting(Message::Connect));
        assert_eq!(state.setting.url, ip);

        let _ = save(&original_ip);
    }
}
