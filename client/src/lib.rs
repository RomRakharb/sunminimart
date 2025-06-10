pub mod custom_widget;
pub mod screen;

use iced::Element;

use screen::{home, inventory};

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

#[derive(Debug, PartialEq)]
pub enum Screen {
    Home,
    Inventory(inventory::State),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Inventory(inventory::State::default())
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    Home(home::Message),
    Inventory(inventory::Message),
}

impl State {
    pub fn update(&mut self, message: Message) {
        match &self.screen {
            Screen::Home => home::update(self, message),
            Screen::Inventory(_) => inventory::update(self, message),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Home => home::view(),
            Screen::Inventory(state) => inventory::view(state),
        }
    }
}
