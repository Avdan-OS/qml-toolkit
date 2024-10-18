use iced::{widget::column, Element};

pub fn main() -> iced::Result {
    iced::application("Avdan GUI Toolkit", Toolkit::update, Toolkit::view).run()
}

#[derive(Default)]
struct Toolkit {}

#[derive(Debug, Clone)]
enum Message {}

impl Toolkit {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        column![].into()
    }
}
