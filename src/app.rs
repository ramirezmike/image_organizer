use iced::{ pane_grid, PaneGrid, executor, Command, Scrollable, scrollable, Length,
            Column, Row, Subscription, Container, Element, Align, Application, Text };
use iced_native::{ keyboard, Event };

#[path = "style.rs"] mod style;
#[path = "file_io.rs"] mod file_io;
#[path = "image.rs"] mod image;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    Resized(pane_grid::ResizeEvent)
}

#[derive(Debug)]
enum AppView {
    SidePanel(SidePanelState),
    ImageQueue(ImageQueueState),
    ImageDisplay(ImageDisplayState)
}

#[derive(Debug)]
struct SidePanelState { 
    label: String
}
impl SidePanelState {
    fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let scrollable = Scrollable::new(scroll)
            .align_items(Align::Start)
            .push(Text::new(self.label.to_string()).size(30));
            
        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Pane { })
            .center_x()
            .into()
    }
}

#[derive(Debug)]
struct ImageQueueState {}
impl ImageQueueState {
    fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let mut row = Row::<'_, Message>::new();
        match file_io::get_directory_list("images/") {
            Ok(x) => {
                row = x.iter().fold(row, |r, image_path| {
                       let text = Text::new(image_path.to_string());
                       let column = Column::<'_, Message>::new().push(text);

                       r.push(Container::new(column)
                        .width(Length::Shrink)
                        .height(Length::Fill)
                        .style(style::ImageQueueItem { }))
                })
            }
            Err(..) => panic!("Error getting file list")
        };

        let scrollable = Scrollable::new(scroll)
                           .width(Length::Fill)
                           .height(Length::Fill)
                           .push(row)
                           .align_items(Align::Start);

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::ImageQueue {})
            .center_y()
            .center_x()
            .into()
    }
}

#[derive(Debug)]
struct ImageDisplayState {
    label: String,
    show_image: bool
}
impl ImageDisplayState {
    fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let mut scrollable = Scrollable::new(scroll)
                                .align_items(Align::Start)
                                .push(Text::new(self.label.to_string()).size(30));

        if self.show_image {
            scrollable = scrollable.push(image::load_image("images/test.jpg".to_string()));
        }
            
        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}

#[derive(Debug)]
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
            AppView::SidePanel(state) => state.view(&mut self.scroll),
            AppView::ImageQueue(state) => state.view(&mut self.scroll),
            AppView::ImageDisplay(state) => state.view(&mut self.scroll)
        }
    }
}

pub struct App {
    state: pane_grid::State<Content>,
    side_panel: pane_grid::Pane,
    image_queue: pane_grid::Pane,
    image_display: pane_grid::Pane
}

impl App {
    fn handle_keyboard_event(self: &mut Self, event: keyboard::Event) {
        match event {
            keyboard::Event::CharacterReceived(character) => {
                if let Some(x) = self.state.get_mut(&self.side_panel) {
                    if let AppView::SidePanel(state) = &mut x.app_view { 
                        state.label = state.label.to_string() + &character.to_string();
                        // TODO handle adding a tag
                    }
                }

                if let Some(x) = self.state.get_mut(&self.image_display) {
                    if let AppView::ImageDisplay(state) = &mut x.app_view { 
                        // TODO handle tagging an image here
                        // currently using this to hide/show images
                        state.show_image = !state.show_image;
                    }
                }

                if let Some(x) = self.state.get_mut(&self.image_queue) {
                    if let AppView::ImageQueue(_)  = &mut x.app_view { 
                        // TODO handle scrolling in queue
                    }
                }
            }
            _ => ()
        }
    }

    fn handle_event(self: &mut Self, event: Event) {
        match event {
            Event::Keyboard(keyboard_event) => self.handle_keyboard_event(keyboard_event),
            _ => ()
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let pane_content = Content::new(AppView::SidePanel(SidePanelState {
            label: String::from("Tags")
        }));
        let state_and_pane = pane_grid::State::new(pane_content);
        let mut state = state_and_pane.0;
        let pane = state_and_pane.1;

        let image_queue_content = Content::new(AppView::ImageQueue(ImageQueueState {}));
        let image_display_content = Content::new(AppView::ImageDisplay(ImageDisplayState {
            label: String::from("Image"),
            show_image: false // starting as false because of image load delay
        }));

        let image_queue_pane;
        let image_queue_split;
        match state.split(pane_grid::Axis::Horizontal, &pane, image_queue_content) {
            Some(x) => {
                image_queue_pane = x.0;
                image_queue_split = x.1;
            }
            None => panic!("Pane couldn't split")
        }

        let image_display_pane;
        let image_display_split;
        match state.split(pane_grid::Axis::Vertical, &pane, image_display_content) {
            Some(x) => {
                image_display_pane = x.0;
                image_display_split = x.1;
            }
            None => panic!("Pane couldn't split")
        }

        // setup starting sizes of views
        state.resize(&image_queue_split, 0.9);
        state.resize(&image_display_split, 0.1);

        let app = App { 
            state: state,
            side_panel: pane,
            image_queue: image_queue_pane,
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
                self.handle_event(event);
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
        .on_resize(Message::Resized);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::MainWindow { })
            .into()
    }
}
