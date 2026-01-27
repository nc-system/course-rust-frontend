use iced::Theme;

#[derive(Debug, Clone)]
enum Message {
    Text,
}

struct State {
    value: String,
}

pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .theme(theme)
        .run()
}

fn new() -> State {
    // ...
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}
