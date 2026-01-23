
use iced::widget::{button, column, text};
use iced::Element;

#[derive(Default)]
struct AppState {
    
}

#[derive(Debug)]
enum Message {
    Exit,
}

fn update(state: &mut AppState, message: Message) {

}

fn view(state: &AppState) -> Element<Message>  {
    text("Hello Iced").into()
}

fn main() -> iced::Result {
    iced::run(update, view)
}
