use iced::{
    self, Element,
    widget::{button, column, text},
};

fn main() -> iced::Result {
    iced::run("Sunminimart", State::update, State::view)
}

#[derive(Default)]
struct State {
    value: i32,
}

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
        .into()
    }
}
