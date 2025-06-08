use iced::widget::{column, text};
use iced::{self, Element, Font};

use sunminimart::screen::{home, inventory};
use sunminimart::{Message, Screen, State};

fn main() -> iced::Result {
    iced::application("Sunminimart", update, view)
        .font(include_bytes!("../asset/Sarabun-Regular.ttf"))
        .default_font(Font::with_name("Sarabun"))
        .run()
}

fn update(state: &mut State, message: Message) {
    match &state.screen {
        Screen::Home => home::update(state, message),
        Screen::Inventory(inventory) => {}
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::Home => home::view(),
        Screen::Inventory(inventory) => column![text("text")].into(),
    }
}
