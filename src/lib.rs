pub mod custom_widget;
pub mod screen;

use screen::{home, inventory};

#[derive(Default)]
pub struct State {
    pub screen: Screen,
}

#[derive(Default, Debug, PartialEq)]
pub enum Screen {
    #[default]
    Home,
    Inventory(inventory::State),
}

#[derive(Clone, Debug)]
pub enum Message {
    Home(home::Message),
    Inventory,
}
