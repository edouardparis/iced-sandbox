use iced::pure::{button, column, row, text, Element, Sandbox};
use iced::{Alignment, Length, Settings};
use iced_sandbox::collapse::collapse;

use std::time::Instant;

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
    events: Vec<Instant>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    DeleteEvent(usize),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 0,
            events: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Counter with collapse")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                self.events.push(Instant::now());
            }
            Message::DecrementPressed => {
                self.value -= 1;
                self.events.push(Instant::now());
            }
            Message::DeleteEvent(i) => {
                self.events.remove(i);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column()
            .padding(20)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                row()
                    .padding(20)
                    .spacing(20)
                    .align_items(Alignment::Center)
                    .push(button("Increment").on_press(Message::IncrementPressed))
                    .push(text(self.value.to_string()).size(50))
                    .push(button("Decrement").on_press(Message::DecrementPressed)),
            )
            .push(collapse::<_, Message, _, _, _>(
                || text(format!("{} events", self.events.len())).into(),
                || {
                    self.events
                        .iter()
                        .enumerate()
                        .fold(column(), |col, (i, event)| {
                            col.push(
                                row()
                                    .padding(10)
                                    .spacing(10)
                                    .align_items(Alignment::Center)
                                    .push(text(format!(
                                        "#{}, {} s ago",
                                        i,
                                        event.elapsed().as_secs()
                                    )))
                                    .push(button("Delete").on_press(Message::DeleteEvent(i))),
                            )
                        })
                        .into()
                },
            ))
            .into()
    }
}
