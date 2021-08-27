use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};
use std::fmt::{Debug, Formatter, Error};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Clone, Copy)]
struct Message(fn(&mut Counter));

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Message").finish_non_exhaustive()
    }
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        let Message(f) = message;
        f(self)
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message({
                        fn f(c: &mut Counter) { c.value += 1; } f
                    })),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message({
                        fn f(c: &mut Counter) { c.value -= 1; } f
                    })),
            )
            .into()
    }
}
