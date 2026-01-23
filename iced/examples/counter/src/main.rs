
use iced::Element;
use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct Counter {
    value: u64,
}

fn main() -> iced::Result {
    iced::run(update, view)
}

fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.value += 1,
    }
}

fn view(counter: &Counter) -> Element<'_, Message>  {
    column![
        text("Counter Application"),
        text(counter.value).size(20),
        button("Increment").on_press(Message::Increment),
        text("Click the button to increment the counter:"),
    ]
    .spacing(10)
    .into()

    // button(text(counter.value)).on_press(Message::Increment).into()
}
