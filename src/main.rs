use iced::{ pane_grid, PaneGrid, executor, Command, Scrollable, scrollable, Length,
            Subscription, Container, Element, Align, Application, Text, Settings };
use iced_native::{ keyboard, Event };

#[derive(Debug, Clone)]
enum Message {
    EventOccurred(Event),
    Resized(pane_grid::ResizeEvent)
}

struct App {
    state: pane_grid::State<Content>,
    side_panel: pane_grid::Pane,
    image_display: pane_grid::Pane
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let pane_content = Content::new(AppView::SidePanel {
            label: String::from("Tags")
        });
        let state_and_pane = pane_grid::State::new(pane_content);
        let mut state = state_and_pane.0;
        let pane = state_and_pane.1;

        let image_display_content = Content::new(AppView::ImageDisplay {
            label: String::from("Image")
        });
        let image_display_pane;
        let image_display_split;
        match state.split(pane_grid::Axis::Vertical, &pane, image_display_content) {
            Some(x) => {
                image_display_pane = x.0;
                image_display_split = x.1;
            }
            None => panic!("Pane couldn't split")
        }

        state.resize(&image_display_split, 0.1);

        let app = App { 
            state: state,
            side_panel: pane,
            image_display: image_display_pane 
        }; 

        (app, Command::none())
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
                            keyboard::Event::KeyPressed { .. }  => (),
                            keyboard::Event::KeyReleased { .. }  => (),
                            keyboard::Event::CharacterReceived(c) => {
                                if let Some(x) = self.state.get_mut(&self.side_panel) {
                                    if let AppView::SidePanel { ref mut label } = &mut x.app_view { 
                                        *label = label.to_string() + &c.to_string();
                                    }
                                }

                                if let Some(x) = self.state.get_mut(&self.image_display) {
                                    if let AppView::ImageDisplay { ref mut label } = &mut x.app_view { 
                                        *label = label.to_string() + &c.to_string() + &String::from("?");
                                    }
                                }
                            }
                        }
                    }
                    _ => ()
                }
            }
            Message::Resized(event) => {
                self.state.resize(&event.split, event.ratio)
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn view(&mut self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&mut self.state, |pane, content, _focus| {
            content.view(pane)
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_resize(Message::Resized)
        .spacing(10);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

enum AppView {
    SidePanel {
        label: String
    },
    ImageDisplay {
        label: String
    }
}

struct Content {
    app_view: AppView,
    scroll: scrollable::State
}

impl Content {
    fn new(app_view: AppView) -> Self {
        Content { 
            app_view: app_view,
            scroll: scrollable::State::new()
        }
    }

    fn view(&mut self, _pane: pane_grid::Pane) -> Element<Message> {
        match &self.app_view {
            AppView::SidePanel { label, .. } => {
                let scrollable = Scrollable::new(&mut self.scroll)
                    .align_items(Align::Start)
                    .push(Text::new(label.to_string()).size(30));
                    
                Container::new(scrollable)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .into()
            },
            AppView::ImageDisplay { label, .. } => {
                let scrollable = Scrollable::new(&mut self.scroll)
                    .align_items(Align::Start)
                    .push(Text::new(label.to_string()).size(30));
                    
                Container::new(scrollable)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .into()
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    App::run(Settings::default()); 
}
