use iced::{ pane_grid, PaneGrid, executor, Command, Scrollable, scrollable, Length,
            Image, 
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
                            keyboard::Event::CharacterReceived(character) => {
                                if let Some(x) = self.state.get_mut(&self.side_panel) {
                                    if let AppView::SidePanel { ref mut label } = &mut x.app_view { 
                                        *label = label.to_string() + &character.to_string();
                                        // TODO handle adding a tag
                                    }
                                }

                                if let Some(x) = self.state.get_mut(&self.image_display) {
                                    if let AppView::ImageDisplay { .. } = &mut x.app_view { 
                                        // TODO handle tagging an image here
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
            .style(style::MainWindow { })
            .padding(10)
            .into()
    }
}

enum AppView {
    SidePanel {
        label: String
    },
    ImageQueue { 
        // TODO split a pane and implement bottom queue view across entire width 
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
                    .style(style::Pane { })
                    .center_x()
                    .into()
            },
            AppView::ImageQueue { .. } => {
                let scrollable = Scrollable::new(&mut self.scroll)
                    .align_items(Align::Start);
                    
                Container::new(scrollable)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .center_x()
                    .into()
            },
            AppView::ImageDisplay { label, .. } => {
                let scrollable = Scrollable::new(&mut self.scroll)
                    .align_items(Align::Start)
                    .push(Text::new(label.to_string()).size(30))
                    .push(load_image("images/test.jpg".to_string()));
                    
                Container::new(scrollable)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_y()
                    .center_x()
                    .into()
            }
        }
    }
}

fn load_image<'a>(src: String) -> Container<'a, Message> {
    Container::new(
        // This implementation was based on the "tour" example
        // https://github.com/hecrj/iced/blob/master/examples/tour/src/main.rs
        // At the time, it said the following:
        //
        // This should go away once we unify resource loading on native platforms
        //
        if cfg!(target_arch = "wasm32") {
            Image::new(src)
        } else {
            Image::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), src))
        }
        .width(Length::Fill)  // TODO: Not sure if it would be handled here or in resize
        .height(Length::Fill) // but it'd be good to make the images resize correctly when the window
                              // resizes so it doesn't introduce scrolling (maybe don't put this in a 
                              // scrollable container?)
    )
    .width(Length::Fill)
    .center_x()
}

//TODO: move this to another file
mod style {
    use iced::{container, Background, Color};

    const BACKGROUND: Color = Color::from_rgb(
        0x1F as f32 / 255.0,
        0x24 as f32 / 255.0,
        0x30 as f32 / 255.0,
    );
    const TEXT : Color = Color::from_rgb(
        0xCB as f32 / 255.0,
        0xCC as f32 / 255.0,
        0xC6 as f32 / 255.0,
    );

    pub struct MainWindow { }
    impl container::StyleSheet for MainWindow {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(TEXT),
                background: Some(Background::Color(BACKGROUND)),
                ..Default::default()
            }
        }
    }

    pub struct Pane { }
    impl container::StyleSheet for Pane {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(TEXT),
                background: Some(Background::Color(BACKGROUND)),
                border_width: 2,
                border_color: Color {
                    a: 0.3,
                    ..Color::BLACK
                },
                ..Default::default()
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    App::run(Settings::default()); 
}
