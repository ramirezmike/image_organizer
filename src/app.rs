use iced::{ pane_grid, executor, Command, Length, Subscription, Container, Element, Application };
use iced_native::{ keyboard, Event };
use std::{ path::PathBuf, fs, collections::HashMap, path, os::unix, env, cell::RefCell, rc::Rc };
use crate::style;
use crate::models::*;
use crate::states::*;
use crate::views::{ MainView, MenuView };

/*
    TODO: 
    - Consider App struct storing all states vs Views holding states?
    - Add some view of other file types 
    - Styling
    - Save/Load to pause and resume
    - Folder navigation

    TODO: List of Known Bugs
*/

pub struct App {
    pub organize_mode: OrganizeMode,
    pub console_messages: RefCell::<Vec::<String>>,
    pub app_state: AppState,
    pub image_queue: pane_grid::Pane,
    pub pane_state: pane_grid::State<MainView>,
    pub tag_input: Option<pane_grid::Pane>,
    pub image_display: pane_grid::Pane,
    pub keyboard_state: KeyboardState,
    pub working_directory: Rc::<RefCell::<String>>,

    side_panel: pane_grid::Pane,
}

impl App {
    pub fn log(self: &Self, message: String) {
        self.console_messages.borrow_mut().push(message);
    }

    pub fn get_mut_state(self: &mut Self, pane: pane_grid::Pane) -> &mut AppView {
        &mut self.pane_state.get_mut(&pane)
                            .expect("Image Queue State missing")
                            .app_view
    }

    pub fn get_state(self: &Self, pane: pane_grid::Pane) -> &AppView {
        &self.pane_state.get(&pane)
                        .expect("Image Queue State missing")
                        .app_view
    }

    pub fn run_organize_process(self: &mut Self) -> Result<(), std::io::Error> {
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
            if !path::Path::new(&key).exists() {
                if let Err(e) = fs::create_dir_all(&key) {
                    self.log(format!("Error creating {}: {}", key, e));
                    continue; // try next folder
                }
            }

            for file in &store[key] {
                let mut source = current_dir.clone();
                source.push(file);
                println!("Moving {}", &(key.to_string() + "||" + file));
                println!("to {}", &(key.to_string() + "/" + file));
                match self.organize_mode {
                    OrganizeMode::Copy => {
                        match fs::copy(source, &(key.to_string() + "/" + file)) {
                            Ok(_) => self.log(format!("{} copied", file)),
                            Err(e) => self.log(format!("Error copying {}: {}", file, e))
                        }
                    }
                    OrganizeMode::Move => {
                        match fs::copy(&source, &(key.to_string() + "/" + file)) {
                            Ok(_) => self.log(format!("{} copied", file)),
                            Err(e) => self.log(format!("Error copying {}: {}", file, e))
                        }
                        match fs::remove_file(&source) {
                            Ok(_) => self.log(format!("Original {}", file)),
                            Err(e) => self.log(format!("{}", e)),
                        }
                    }
                    OrganizeMode::Link => {
                        match unix::fs::symlink(source, &(key.to_string() + "/" + file)) {
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
        match self.app_state {
            AppState::Menu => MenuView::handle_keyboard(self, event),
            AppState::Tagging => MainView::handle_keyboard(self, event)
        }
    }

    fn handle_event(self: &mut Self, event: Event) {
        match event {
            Event::Keyboard(keyboard_event) => self.handle_keyboard_event(keyboard_event),
            _ => ()
        }
    }

    fn load_current_image(self: &mut Self) {
        if let Some((current_path, tags)) = self.get_current_image_info() {
            let tags = tags.iter().map(|tag| *tag.clone()).collect();

            let display_state = self.get_mut_state(self.image_display).image_display_mut();
            display_state.current_image_path = current_path;
            display_state.current_image_tags = Some(tags);
        }
    }

    fn get_current_image_info(self: &Self) -> Option<(String, Vec::<&char>)> {
        let state = self.get_state(self.image_queue).image_queue();

        if !state.image_infos.is_empty() {
            Some((state.image_infos[state.selected_image_index].path.clone(),
                 state.image_infos[state.selected_image_index].tags.keys().collect()))
        } else {
            None
        }
    }

    pub fn toggle_tag_on_current_image(self: &mut Self, key: &char) {
        let state = self.get_mut_state(self.image_queue).image_queue_mut();
        if state.image_infos[state.selected_image_index].tags.contains_key(key) {
            state.image_infos[state.selected_image_index].tags.remove(key);
        } else {
            state.image_infos[state.selected_image_index].tags.insert(key.clone(), ());
        }
    }

    pub fn does_tag_exist(self: &Self, key: &String) -> bool {
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
        let working_directory: String;
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            let error_message = "Error getting current directory";
            working_directory = String::from(std::env::current_dir().expect(error_message)
                                                                    .to_str()
                                                                    .expect(error_message));
        } else {
            working_directory = args[1].clone();
        }

        assert!(env::set_current_dir(&PathBuf::from(&working_directory)).is_ok());
        let working_directory: Rc::<RefCell::<String>> = Rc::new(RefCell::new(working_directory));

        let pane_content = MainView::new(AppView::SidePanel(SidePanelState {
            label: String::from("Tags"),
            tags: HashMap::<String,String>::new()
        }));
        let image_queue_content = MainView::new(AppView::ImageQueue(ImageQueueState::new()));
        let image_display_content = MainView::new(AppView::ImageDisplay(ImageDisplayState {
            root_path: Rc::clone(&working_directory),
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
            pane_state: state,
            side_panel: pane,
            image_queue: image_queue_pane,
            image_display: image_display_pane,
            tag_input: None,
            keyboard_state: KeyboardState::Tagging,
            organize_mode: OrganizeMode::Copy,
            console_messages: RefCell::new(Vec::<String>::new()),
            working_directory: working_directory
        }, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                self.handle_event(event);
            }
            Message::Resized(event) => {
                self.pane_state.resize(&event.split, event.ratio)
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
                        self.pane_state.close(&tag_input);
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
                Container::new(MenuView::view(self))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(style::MainWindow { })
                    .into()
            }
            AppState::Tagging => {
                Container::new(MainView::view(&mut self.pane_state))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(5)
                    .style(style::MainWindow { })
                    .into()
            }
        }
    }
}
