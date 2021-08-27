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
struct Callback(fn(&mut Counter));

impl Debug for Callback {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Callback").finish_non_exhaustive()
    }
}

impl Sandbox for Counter {
    type Message = Callback;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) {
        let Callback(f) = message;
        f(self)
    }

    fn view(&mut self) -> Element<Self::Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Callback(|c| c.value += 1))
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Callback(|c| c.value -= 1))
            )
            .into()
    }
}
