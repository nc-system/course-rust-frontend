
use iced::Element;
use iced::widget::{container, text};

#[derive(Debug, Clone)]
enum Message {
    Text
}

#[derive(Default)]
struct State {
    content: String,
}

fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(state: &mut State, message: Message)  {
    match message {
        Message::Text => state.content.push_str("I am 300px tall!"),
    }
}

fn view(state: &State) -> Element<Message> {
    container(
        text(&state.content).height(300)
    )
    .into()
}