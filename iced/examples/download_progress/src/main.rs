use iced::widget::{button, column, progress_bar, text};
use iced::{Element, Length, Task};

pub fn main() -> iced::Result {
    iced::run(update, view)
}

#[derive(Default)]
pub struct Counter {
    value: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    ResetPressed,
}

fn update(counter: &mut Counter, message: Message) -> Task<Message> {
    match message {
        Message::IncrementPressed => {
            counter.value += 10.0;
            if counter.value > 100.0 {
                counter.value = 100.0;
            }
        }
        Message::ResetPressed => {
            counter.value = 0.0;
        }
    }
    Task::none()
}

fn view(counter: &Counter) -> Element<Message> {
    let progress = progress_bar(0.0..=100.0, counter.value);
    
    column![
        text("Download Progress").size(32),
        progress,
        text(format!("{:.0}%", counter.value)),
        button("Increment").on_press(Message::IncrementPressed),
        button("Reset").on_press(Message::ResetPressed),
    ]
    .padding(20)
    .spacing(20)
    .width(Length::Fill)
    .into()
}