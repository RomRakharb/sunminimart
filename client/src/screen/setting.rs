use iced::{
    Element, Length, Pixels, Subscription,
    widget::{button, container, horizontal_space, row, text_input},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

pub(crate) static PATH: &str = "./asset/setting.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct State {
    pub(crate) server_ip: String,
}

impl Default for State {
    fn default() -> Self {
        let default_setting = Self {
            server_ip: "".to_string(),
        };

        let create_file = || -> std::io::Result<()> {
            let json_data = serde_json::to_string_pretty(&default_setting).unwrap();
            let mut file = fs::File::create(PATH)?;
            file.write_all(json_data.as_bytes())?;
            Ok(())
        };

        if let Ok(data) = fs::read_to_string(PATH) {
            if let Ok(setting) = serde_json::from_str(&data) {
                setting
            } else {
                let _ = create_file;
                default_setting
            }
        } else {
            let _ = create_file;
            default_setting
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    OnIPChange(String),
    Submit,
    Back,
}

pub(crate) fn update(state: &mut crate::State, message: crate::Message) {
    if let crate::Message::Setting(message) = message {
        match message {
            Message::OnIPChange(ip) => {
                state.screen = crate::Screen::Setting(State { server_ip: ip })
            }
            Message::Submit => {
                if let crate::Screen::Setting(State { server_ip }) = &state.screen {
                    state.setting.server_ip = server_ip.to_string();
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
                text_input("", &state.server_ip)
                    .on_input(|input| { crate::Message::Setting(Message::OnIPChange(input)) }),
                button("บันทึก").on_press(crate::Message::Setting(Message::Submit))
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
    Subscription::none()
}

fn _save_setting(setting: State) -> std::io::Result<()> {
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
        state.update(crate::Message::Setting(Message::Back));
        assert_eq!(state.screen, crate::Screen::Home);
    }

    #[test]
    fn change_ip() {
        let ip = "192.168.1.45:3000".to_string();

        let mut state = init_state();
        state.update(crate::Message::Setting(Message::OnIPChange(ip.clone())));
        if let crate::Screen::Setting(state) = &state.screen {
            assert_eq!(state.server_ip, ip);
        } else {
            panic!("Screen suppose to be Setting");
        }
        state.update(crate::Message::Setting(Message::Submit));
        assert_eq!(state.setting.server_ip, ip);
    }
}
