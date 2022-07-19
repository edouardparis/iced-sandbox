use iced::pure::{button, column};
use iced_lazy::pure::{self, Component};
use iced_pure::Element;

pub fn collapse<'a, Message, Renderer>(
    header: impl Fn() -> Element<'a, Message, Renderer> + 'a,
    content: impl Fn() -> Element<'a, Message, Renderer> + 'a,
) -> Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'static,
{
    Collapse {
        header: Box::new(header),
        content: Box::new(content),
    }
    .into()
}

struct Collapse<'a, Message, Renderer> {
    header: Box<dyn Fn() -> Element<'a, Message, Renderer> + 'a>,
    content: Box<dyn Fn() -> Element<'a, Message, Renderer> + 'a>,
}

#[derive(Debug, Clone, Copy)]
enum Event<T> {
    Internal(T),
    Collapse(bool),
}

impl<'a, Message, Renderer> Component<Message, Renderer> for Collapse<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'static,
{
    type State = bool;
    type Event = Event<Message>;

    fn update(&mut self, state: &mut Self::State, event: Event<Message>) -> Option<Message> {
        match event {
            Event::Internal(e) => Some(e),
            Event::Collapse(s) => {
                *state = s;
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<Self::Event, Renderer> {
        if *state {
            column()
                .push(button((self.header)().map(Event::Internal)).on_press(Event::Collapse(false)))
                .push((self.content)().map(Event::Internal))
                .into()
        } else {
            column()
                .push(button((self.header)().map(Event::Internal)).on_press(Event::Collapse(true)))
                .into()
        }
    }
}
impl<'a, Message, Renderer> From<Collapse<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_native::Renderer + 'static,
{
    fn from(c: Collapse<'a, Message, Renderer>) -> Self {
        pure::component(c)
    }
}