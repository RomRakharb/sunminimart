pub(crate) mod custom_component;
pub(crate) mod screen;

use iced::{Element, Subscription, Task};

use screen::setting::State as Setting;
use screen::{home, inventory, setting};

#[derive(Default, Debug)]
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
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self.screen {
            Screen::Home => home::update(self, message),
            Screen::Inventory(_) => inventory::update(self, message),
            Screen::Setting(_) => {
                setting::update(self, message);
                Task::none()
            }
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

pub fn subscription(state: &State) -> Subscription<Message> {
    match &state.screen {
        Screen::Home => Subscription::none(),
        Screen::Setting(state) => setting::subscription(state),
        Screen::Inventory(state) => inventory::subscription(state),
    }
}
