pub(crate) mod custom_widget;
pub(crate) mod screen;

use iced::Element;

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

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Setting {
    server_ip: String,
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
