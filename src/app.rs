use iced::{ pane_grid, PaneGrid, executor, Command, Length, 
            Subscription, Container, Element, Application };
use iced_native::{ keyboard, Event };
use std::{ cmp, collections::HashMap };
mod style;
mod utils;
mod content;
mod states;
use states::image_queue::ImageQueueState as ImageQueueState;
use states::image_display::ImageDisplayState as ImageDisplayState;
use states::side_panel::SidePanelState as SidePanelState;
use states::tag_input::TagInputState as TagInputState;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    TextInputChanged(String),
    TextInputSubmitted,
    Resized(pane_grid::ResizeEvent)
}

#[derive(Debug)]
pub enum AppView {
    SidePanel(SidePanelState),
    ImageQueue(ImageQueueState),
    ImageDisplay(ImageDisplayState),
    TagInput(TagInputState),
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
    fn get_state(self: &mut Self, pane: pane_grid::Pane) -> &mut AppView {
        &mut self.state.get_mut(&pane)
                       .expect("Image Queue State missing")
                       .app_view
    }

    fn handle_keyboard_event(self: &mut Self, event: keyboard::Event) {
        match self.keyboard_state {
            KeyboardState::Tagging => {
                match event {
                    keyboard::Event::KeyPressed { key_code, .. } => {
                        if let AppView::ImageQueue(state) = self.get_state(self.image_queue) { 
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
                    keyboard::Event::CharacterReceived(character) => {
                        if character.is_alphabetic() && !self.does_tag_exist(&character.to_string()) {
                            self.keyboard_state = KeyboardState::CreatingTag;
                            let tag_input_content = content::Content::new(AppView::TagInput(TagInputState { 
                                tag_input_value: "".to_string(),
                                tag: character
                            }));

                            let (pane, split) = self.state.split(pane_grid::Axis::Horizontal, 
                                                                 &self.image_display, tag_input_content)
                                                          .expect("Pane couldn't split");
                            self.tag_input = Some(pane);
                            self.state.resize(&split, 0.9);
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
        if let AppView::ImageDisplay(state) = self.get_state(self.image_display) { 
            state.current_image_path = current_path;
        }
    }

    fn get_current_image_path(self: &mut Self) -> String {
        let mut result: String = "".to_string();
        if let AppView::ImageQueue(state) = self.get_state(self.image_queue) {
            result = state.image_paths[state.selected_image_index].clone();
        }

        return result;
    }

    fn does_tag_exist(self: &mut Self, key: &String) -> bool {
        let mut result: bool = false;

        if let AppView::SidePanel(state) = self.get_state(self.side_panel) {
            result = state.tags.contains_key(key);
        }

        return result;
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Message>) {
        let pane_content = content::Content::new(AppView::SidePanel(SidePanelState {
            label: String::from("Tags"),
            tags: HashMap::<String,String>::new()
        }));
        let image_queue_content = content::Content::new(AppView::ImageQueue(ImageQueueState::new("images/")));
        let image_display_content = content::Content::new(AppView::ImageDisplay(ImageDisplayState {
            label: String::from("Image"),
            current_image_path: "".to_string()
        }));

        let (mut state, pane) = pane_grid::State::new(pane_content);

        let (image_queue_pane, image_queue_split) = state.split(pane_grid::Axis::Horizontal, 
                                                                &pane, image_queue_content)
                                                         .expect("Pane couldn't split"); 
        let (image_display_pane, image_display_split) = state.split(pane_grid::Axis::Vertical, 
                                                                    &pane, image_display_content) 
                                                             .expect("Pane couldn't split");

        // setup starting sizes of views
        state.resize(&image_queue_split, 0.9);
        state.resize(&image_display_split, 0.1);

        (App { 
            state: state,
            side_panel: pane,
            image_queue: image_queue_pane,
            image_display: image_display_pane,
            tag_input: None,
            keyboard_state: KeyboardState::Tagging
        }, Command::none())
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
                        if let AppView::TagInput(state) = self.get_state(tag_input) { 
                            state.tag_input_value = text;
                        }
                    },
                    None => ()
                }
            }
            Message::TextInputSubmitted => {
                let mut submitted_value: String = "Error getting submitted value".to_string();
                let mut submitted_tag: String = "Error getting tag".to_string();
                match self.tag_input {
                    Some(tag_input) => {
                        if let AppView::TagInput(state) = self.get_state(tag_input) { 
                            submitted_value = state.tag_input_value.clone();
                            submitted_tag = state.tag.to_string();
                            state.tag_input_value = "".to_string();
                            self.state.close(&tag_input);
                        }
                    },
                    _ => ()
                }

                // code for inserting a new tag into hashmap
                if let AppView::SidePanel(state) = self.get_state(self.side_panel) { 
                    state.tags.insert(submitted_tag, submitted_value);
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
