use iced::{ pane_grid, PaneGrid, executor, Command, Length, Text, Column,
            Row, Subscription, Container, Element, Application, Radio };
use iced_native::{ keyboard, Event };
use std::{ fs, collections::HashMap, path, os::unix, env };
use crate::style;
use crate::content;
use crate::lib_ext::*;
use crate::state::*;
use crate::enums::*;

const TEST_DIRECTORY: &str = "images/";

/*
    TODO: Bugs
    - able to tag with characters that can't be folder names
    - { } doesn't go to the next/prev untagged
    - while tagging, image navigation still responds
*/

pub struct App {
    app_state: AppState,
    state: pane_grid::State<content::Content>,
    side_panel: pane_grid::Pane,
    image_queue: pane_grid::Pane,
    image_display: pane_grid::Pane,
    tag_input: Option<pane_grid::Pane>,
    keyboard_state: KeyboardState,
    organize_mode: OrganizeMode,
    console_messages: Vec::<String>
}

impl App {
    fn log(self: &mut Self, message: String) {
        self.console_messages.push(message);
    }

    fn get_mut_state(self: &mut Self, pane: pane_grid::Pane) -> &mut AppView {
        &mut self.state.get_mut(&pane)
                       .expect("Image Queue State missing")
                       .app_view
    }

    fn get_state(self: &Self, pane: pane_grid::Pane) -> &AppView {
        &self.state.get(&pane)
                   .expect("Image Queue State missing")
                   .app_view
    }

    fn run_organize_process(self: &mut Self) -> Result<(), std::io::Error> {
        let side_panel = self.get_state(self.side_panel).side_panel();
        let image_queue = self.get_state(self.image_queue).image_queue();

        let mut store = HashMap::<String, Vec::<String>>::new();
        store = image_queue.image_infos.iter().fold(store, |mut acc, image_info| {
            for tag in image_info.tags.iter() {
                let tag = &tag.0.to_string();
                if let Some(tag_label) = side_panel.tags.get(tag) {
                    if !acc.contains_key(tag_label) {
                        acc.insert(tag_label.to_string(), Vec::<String>::new());
                    }

                    if let Some(tag_store) = acc.get_mut(tag_label) {
                        tag_store.push(image_info.path.clone());
                    }
                }
            }

            acc
        });

        let current_dir = env::current_dir()?;
        for key in store.keys() {
            let current_directory = TEST_DIRECTORY.to_string() + key;
            if !path::Path::new(&current_directory).exists() {
                if let Err(e) = fs::create_dir_all(&current_directory) {
                    self.log(format!("Error creating {}: {}", current_directory, e));
                    continue; // try next folder
                }
            }

            for file in &store[key] {
                let mut source = current_dir.clone();
                source.push("images");
                source.push(file);
                match self.organize_mode {
                    OrganizeMode::Copy => {
                        match fs::copy(source,
                                       &(current_directory.to_string() 
                                           + "/" + file)) {
                            Ok(_) => self.log(format!("{} copied", file)),
                            Err(e) => self.log(format!("Error copying {}: {}", file, e))
                        }
                    }
                    OrganizeMode::Move => {
                        match fs::copy(&source,
                                       &(current_directory.to_string() 
                                           + "/" + file)) {
                            Ok(_) => self.log(format!("{} copied", file)),
                            Err(e) => self.log(format!("Error copying {}: {}", file, e))
                        }
                        match fs::remove_file(&source) {
                            Ok(_) => self.log(format!("Original {}", file)),
                            Err(e) => self.log(format!("{}", e)),
                        }
                    }
                    OrganizeMode::Link => {
                        match unix::fs::symlink(source,
                                            &(current_directory.to_string() 
                                                + "/" + file)) {
                            Ok(_) => self.log(format!("{} linked", file)),
                            Err(e) => self.log(format!("Error linking {}: {}", file, e))
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    fn handle_keyboard_event(self: &mut Self, event: keyboard::Event) {
        match event {
            keyboard::Event::KeyPressed { key_code, modifiers } => {
                let is_shift_pressed = &modifiers.shift;
                match self.app_state {
                    AppState::Menu => {
                        match key_code {
                            keyboard::KeyCode::Escape => {
                                self.app_state = AppState::Tagging 
                            }
                            keyboard::KeyCode::Q => {
                                // TODO: exit more gracefully
                                assert!(1 == 0); // ¯\_(ツ)_/¯
                            }
                            keyboard::KeyCode::O => {
                                self.organize_mode.next();
                            }
                            keyboard::KeyCode::R => {
                                match self.run_organize_process() {
                                    Err(e) => panic!("Error running process: {}", e),
                                    _ => ()
                                }
                            }
                            keyboard::KeyCode::C => {
                                self.console_messages.clear();
                            }
                            _ => ()
                        }
                    }
                    AppState::Tagging => {
                        match key_code {
                            keyboard::KeyCode::Escape => {
                                self.app_state = AppState::Menu
                            }
                            _ => ()
                        }

                        let state = self.get_mut_state(self.image_queue).image_queue_mut();
                        match key_code {
                            keyboard::KeyCode::Left => {
                                match state.image_infos.prev(state.selected_image_index, |_| true) {
                                    Some(x) => state.selected_image_index = x,
                                    _ => ()
                                }
                            },
                            keyboard::KeyCode::Right => {
                                match state.image_infos.next(state.selected_image_index, |_| true) {
                                    Some(x) => state.selected_image_index = x,
                                    _ => ()
                                }
                            },
                            keyboard::KeyCode::LBracket => {
                                match state.image_infos.prev(state.selected_image_index,
                                                             |x| x.tags.is_empty() == !is_shift_pressed) {
                                    Some(x) => state.selected_image_index = x,
                                    _ => ()
                                }
                            },
                            keyboard::KeyCode::RBracket => {
                                match state.image_infos.next(state.selected_image_index,
                                                             |x| x.tags.is_empty() == !is_shift_pressed) {
                                    Some(x) => state.selected_image_index = x,
                                    _ => ()
                                }
                            },
                            keyboard::KeyCode::Delete => {
                                if state.selected_image_index < state.image_infos.len() {
                                    if let Some(x) = state.image_infos.get(state.selected_image_index) {
                                        match fs::remove_file(TEST_DIRECTORY.to_string() + &x.path) {
                                            Ok(_) => println!("Deleted {} successfully", x.path),
                                            Err(e) => println!("{}", e),
                                        }
                                        state.delete_current();
                                    }
                                }
                            },
                            _ => ()
                        }
                    }
                }
            }
            keyboard::Event::CharacterReceived(character) => {
                match self.app_state {
                    AppState::Tagging => {
                        match self.keyboard_state {
                            KeyboardState::Tagging => { 
                                if character.is_alphabetic() {
                                    if !self.does_tag_exist(&character.to_string()) {
                                        self.keyboard_state = KeyboardState::None;
                                        let tag_input_content = content::Content::new(AppView::TagInput(TagInputState { 
                                            tag_input_value: "".to_string(),
                                            tag: character
                                        }));

                                        let (pane, split) = self.state.split(pane_grid::Axis::Horizontal, 
                                                                             &self.image_display, tag_input_content)
                                                                      .expect("Pane couldn't split");
                                        self.tag_input = Some(pane);
                                        self.state.resize(&split, 0.9);
                                    }

                                    self.toggle_tag_on_current_image(&character);
                                }
                            }
                            KeyboardState::None => ()
                        }
                    }
                    _ => ()
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

    fn load_current_image(self: &mut Self) {
        let (current_path, tags) = self.get_current_image_info();
        let tags = tags.iter().map(|tag| *tag.clone()).collect();

        let display_state = self.get_mut_state(self.image_display).image_display_mut();
        display_state.current_image_path = current_path;
        display_state.current_image_tags = Some(tags);
    }

    fn get_current_image_info(self: &mut Self) -> (String, Vec::<&char>) {
        let state = self.get_mut_state(self.image_queue).image_queue_mut();

        (state.image_infos[state.selected_image_index].path.clone(),
         state.image_infos[state.selected_image_index].tags.keys().collect())
    }

    fn toggle_tag_on_current_image(self: &mut Self, key: &char) {
        let state = self.get_mut_state(self.image_queue).image_queue_mut();
        if state.image_infos[state.selected_image_index].tags.contains_key(key) {
            state.image_infos[state.selected_image_index].tags.remove(key);
        } else {
            state.image_infos[state.selected_image_index].tags.insert(key.clone(), ());
        }
    }

    fn does_tag_exist(self: &mut Self, key: &String) -> bool {
        self.get_state(self.side_panel)
            .side_panel()
            .tags
            .contains_key(key)
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
        let image_queue_content = content::Content::new(AppView::ImageQueue(ImageQueueState::new(TEST_DIRECTORY)));
        let image_display_content = content::Content::new(AppView::ImageDisplay(ImageDisplayState {
            root_path: TEST_DIRECTORY.to_string(),
            label: String::from("Image"),
            current_image_path: "".to_string(),
            current_image_tags: None
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
            app_state: AppState::Menu,
            state: state,
            side_panel: pane,
            image_queue: image_queue_pane,
            image_display: image_display_pane,
            tag_input: None,
            keyboard_state: KeyboardState::Tagging,
            organize_mode: OrganizeMode::Copy,
            console_messages: Vec::<String>::new()
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
                        let state = self.get_mut_state(tag_input).tag_input_mut();
                        state.tag_input_value = text;
                    },
                    None => ()
                }
            }
            Message::TextInputSubmitted => {
                let mut submitted_value: String = "Error getting submitted value".to_string();
                let mut submitted_tag: String = "Error getting tag".to_string();
                match self.tag_input {
                    Some(tag_input) => {
                        let state = self.get_mut_state(tag_input).tag_input_mut();
                        submitted_value = state.tag_input_value.clone();
                        submitted_tag = state.tag.to_string();
                        state.tag_input_value = "".to_string();
                        self.state.close(&tag_input);
                    },
                    _ => ()
                }

                let state = self.get_mut_state(self.side_panel).side_panel_mut();
                state.tags.insert(submitted_tag, submitted_value);

                self.keyboard_state = KeyboardState::Tagging;
                self.tag_input = None;
            }
            Message::SelectedOrganizeMode(mode) => {
                self.organize_mode = mode;
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
        match self.app_state {
            AppState::Menu => {
                let mut column = Column::<'_, Message>::new()
                                    .push(Row::<'_, Message>::new()
                                        .push(Container::new(Text::new("Q - Quit"))))
                                    .push(Row::<'_, Message>::new()
                                        .push(Container::new(Text::new("O - Organize Mode")))
                                        .push(Radio::new(
                                                OrganizeMode::Copy, 
                                                "Copy", 
                                                Some(self.organize_mode), 
                                                Message::SelectedOrganizeMode))
                                        .push(Radio::new(
                                                OrganizeMode::Move, 
                                                "Move", 
                                                Some(self.organize_mode), 
                                                Message::SelectedOrganizeMode))
                                        .push(Radio::new(
                                                OrganizeMode::Link, 
                                                "Link", 
                                                Some(self.organize_mode), 
                                                Message::SelectedOrganizeMode))
                                        )
                                    .push(Row::<'_, Message>::new()
                                        .push(Container::new(Text::new("R - Run Organize Process"))))
                                    .push(Row::<'_, Message>::new()
                                        .push(Container::new(Text::new("C - Clear Console"))))
                                    .push(Row::<'_, Message>::new()
                                        .push(Container::new(Text::new("Escape - Close Menu"))));

                column = self.console_messages.iter()
                                              .fold(column, |acc, message| {
                                                  let text = Text::new(message);
                                                  let container = Container::new(text);
                                                  let row = Row::<'_, Message>::new()
                                                                .push(container);

                                                  acc.push(row)
                                              });
                Container::new(column)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(style::MainWindow { })
                    .into()
            }
            AppState::Tagging => {
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
    }
}
