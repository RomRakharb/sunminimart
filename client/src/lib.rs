pub(crate) mod custom_widget;
pub(crate) mod screen;

use iced::Element;

use screen::setting::State as Setting;
use screen::{home, inventory, setting};

#[derive(Default)]
pub struct State {
    pub(crate) screen: Screen,
    pub(crate) setting: Setting,
}

#[derive(Debug, PartialEq, Default)]
pub(crate) enum Screen {
    #[default]
    Home,
    Inventory(Box<inventory::State>),
    Setting(setting::State),
}

#[derive(Clone, Debug)]
pub enum Message {
    Home(home::Message),
    Inventory(inventory::Message),
    Setting(setting::Message),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match self.screen {
            Screen::Home => home::update(self, message),
            Screen::Inventory(_) => inventory::update(self, message),
            Screen::Setting(_) => setting::update(self, message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Home => home::view(),
            Screen::Inventory(state) => inventory::view(state),
            Screen::Setting(state) => setting::view(state),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn default_setting() -> std::io::Result<()> {
        let state = State::default();
        let current_setting: Setting = state.setting;

        let _ = fs::remove_file(setting::PATH);
        let default_setting = Setting::default();
        assert_eq!(
            default_setting,
            Setting {
                server_ip: "".to_string()
            }
        );

        let _ = (|| {
            let json_data = serde_json::to_string_pretty(&current_setting).unwrap();
            let mut file = fs::File::create(setting::PATH)?;
            file.write_all(json_data.as_bytes())?;
            Ok::<(), std::io::Error>(())
        })();

        assert_eq!(current_setting, Setting::default());

        Ok(())
    }
}
