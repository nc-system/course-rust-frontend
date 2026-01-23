
use iced::{Fill, Element};
use iced::widget::{column, container, row, text};

#[derive(Debug, Clone)]
enum Message {
    Text,
}

#[derive(Default)]
struct State {
    value: u16,
}

fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Text => state.value += 1,
    }
}

fn view(state: &State) -> Element<'_, Message> {
    container(
        column![
            "Top",
            row!["Left", "Right"].spacing(10),
            row![text(state.value.to_string())].padding(10),
            "Bottom"
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()
}
