use iced::{ executor, Subscription, Command, Element, Column, Align, Application, Text, Settings };
use iced_native::{ input::keyboard, Event, subscription };

#[derive(Default)]
struct App {
    label: String
}

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        (App::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("image_organizer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                println!("{:?}", event);
                match event {
                    Event::Keyboard(keyboard_event) => {
                        match keyboard_event {
                            keyboard::Event::Input { .. }  => (),
                            keyboard::Event::CharacterReceived(c) => self.label = c.to_string()
                        }
                    }
                    _ => ()
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(Text::new(&self.label).size(50))
            .into()
    }
}

fn main() {
    println!("Hello, world!");
    App::run(Settings::default()); 
}
