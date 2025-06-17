use iced::{
    Element, Length,
    widget::{column, container, text_input},
};

#[derive(Debug, PartialEq, Default)]
pub(crate) struct State {
    server_ip: String,
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
    container(column![text_input("", &state.server_ip).on_input(
        |input| crate::Message::Setting(Message::OnIPChange(input))
    )])
    .center(Length::Fill)
    .into()
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
