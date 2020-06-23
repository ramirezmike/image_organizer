use iced::{ pane_grid, PaneGrid, executor, Command, Length, 
            Subscription, Container, Element, Application };
use iced_native::{ keyboard, Event };
use std::{ cmp, collections::HashMap };

mod style;
mod utils;
mod states;
mod content;
use states::{ image_queue, tag_input, image_display, side_panel };

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    TextInputChanged(String),
    TextInputSubmitted,
    Resized(pane_grid::ResizeEvent)
}

#[derive(Debug)]
pub enum AppView {
    SidePanel(side_panel::SidePanelState),
    ImageQueue(image_queue::ImageQueueState),
    ImageDisplay(image_display::ImageDisplayState),
    TagInput(tag_input::TagInputState),
}

#[derive(Debug)]
enum KeyboardState {
    Tagging,
    CreatingTag
}

pub struct App {
    state: pane_grid::State<content::Content>,
    side_panel: pane_grid::Pane,
    image_queue: pane_grid::Pane,
    image_display: pane_grid::Pane,
    tag_input: Option<pane_grid::Pane>,
    keyboard_state: KeyboardState
}

impl App {
    fn handle_keyboard_event(self: &mut Self, event: keyboard::Event) {
        match self.keyboard_state {
            KeyboardState::Tagging => {
                match event {
                    keyboard::Event::KeyPressed { key_code, .. } => {
                        if let Some(x) = self.state.get_mut(&self.image_queue) {
                            if let AppView::ImageQueue(state) = &mut x.app_view { 
                                match key_code {
                                    keyboard::KeyCode::Left => {
                                        if state.selected_image_index == 0 { 
                                            state.selected_image_index = 0; 
                                        } else {
                                            state.selected_image_index = state.selected_image_index - 1;
                                        }
                                    },
                                    keyboard::KeyCode::Right => {
                                        state.selected_image_index = cmp::min(state.image_paths.len() - 1, 
                                                                              state.selected_image_index + 1);
                                    },
                                    _ => ()
                                }
                            }
                        }
                    }
                    keyboard::Event::CharacterReceived(character) => {
                        if character.is_alphabetic() && !self.does_tag_exist(&character.to_string()) {
                            self.keyboard_state = KeyboardState::CreatingTag;
                            let tag_input_content = content::Content::new(AppView::TagInput(states::tag_input::TagInputState { 
                                tag_input_value: "".to_string(),
                                tag: character
                            }));

                            let text_input_split;
                            match self.state.split(pane_grid::Axis::Horizontal, 
                                                   &self.image_display, 
                                                   tag_input_content) {
                                Some(x) => {
                                    self.tag_input = Some(x.0);
                                    text_input_split = x.1;
                                }
                                None => panic!("Pane couldn't split")
                            }

                            self.state.resize(&text_input_split, 0.9);
                        } else {
                            // TODO here we need to add the tag to the image
                        }
                    }
                    _ => ()
                }
            }
            KeyboardState::CreatingTag => ()
        }
    }

    fn handle_event(self: &mut Self, event: Event) {
        match event {
            Event::Keyboard(keyboard_event) => self.handle_keyboard_event(keyboard_event),
            _ => ()
        }
    }

    fn load_current_image(self: &mut Self) {
        let current_path = self.get_current_image_path();
        if let Some(x) = self.state.get_mut(&self.image_display) {
            if let AppView::ImageDisplay(state) = &mut x.app_view { 
                state.current_image_path = current_path;
            }
        }
    }

    fn get_current_image_path(self: &Self) -> String {
        let mut result: String = "".to_string();

        if let Some(x) = self.state.get(&self.image_queue) {
            if let AppView::ImageQueue(state) = &x.app_view { 
                result = state.image_paths[state.selected_image_index].clone();
            }
        }

        return result;
    }

    fn does_tag_exist(self: &Self, key: &String) -> bool {
        let mut result: bool = false;

        if let Some(x) = self.state.get(&self.side_panel) {
            if let AppView::SidePanel(state) = &x.app_view { 
                result = state.tags.contains_key(key);
            }
        }

        return result;
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let pane_content = content::Content::new(AppView::SidePanel(side_panel::SidePanelState {
            label: String::from("Tags"),
            tags: HashMap::<String,String>::new()
        }));
        let state_and_pane = pane_grid::State::new(pane_content);
        let mut state = state_and_pane.0;
        let pane = state_and_pane.1;

        let image_queue_content = content::Content::new(AppView::ImageQueue(image_queue::ImageQueueState::new("images/")));
        let image_display_content = content::Content::new(AppView::ImageDisplay(image_display::ImageDisplayState {
            label: String::from("Image"),
            current_image_path: "".to_string()
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
            image_display: image_display_pane,
            tag_input: None,
            keyboard_state: KeyboardState::Tagging
        }; 

        (app, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                self.handle_event(event);
            }
            Message::Resized(event) => {
                self.state.resize(&event.split, event.ratio)
            }
            Message::TextInputChanged(text) => {
                match self.tag_input {
                    Some(tag_input) => {
                        if let Some(x) = self.state.get_mut(&tag_input) {
                            if let AppView::TagInput(state) = &mut x.app_view { 
                                state.tag_input_value = text;
                            }
                        }
                    },
                    _ => ()
                }
            }
            Message::TextInputSubmitted => {
                let mut submitted_value: String = "Error getting submitted value".to_string();
                let mut submitted_tag: String = "Error getting tag".to_string();
                match self.tag_input {
                    Some(tag_input) => {
                        if let Some(x) = self.state.get_mut(&tag_input) {
                            if let AppView::TagInput(state) = &mut x.app_view { 
                                submitted_value = state.tag_input_value.clone();
                                submitted_tag = state.tag.to_string();
                                state.tag_input_value = "".to_string();
                                self.state.close(&tag_input);
                            }
                        }
                    },
                    _ => ()
                }

                // code for inserting a new tag into hashmap
                if let Some(x) = self.state.get_mut(&self.side_panel) {
                    if let AppView::SidePanel(state) = &mut x.app_view { 
                        state.tags.insert(submitted_tag, submitted_value);
                    }
                }

                self.keyboard_state = KeyboardState::Tagging;
                self.tag_input = None;
            }
        }

        self.load_current_image();

        Command::none()
    }

    fn title(&self) -> String {
        String::from("image_organizer")
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
        .on_resize(10, Message::Resized);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::MainWindow { })
            .into()
    }
}
