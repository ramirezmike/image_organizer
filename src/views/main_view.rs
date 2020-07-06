use iced::{ scrollable, Length, pane_grid, PaneGrid };
use iced_native::{ text_input, keyboard };
use std::{ fs, io };

use crate::app::App;
use crate::states::*;
use crate::models::*;
use crate::lib_ext::*;

#[derive(Debug)]
pub struct MainView {
    pub app_view: AppView,
    scroll: scrollable::State,
    text_input_state: text_input::State
}

impl MainView {
    pub fn new(app_view: AppView) -> Self {
        MainView { 
            app_view: app_view,
            scroll: scrollable::State::new(),
            text_input_state: text_input::State::focused()
        }
    }

    pub fn view<'a>(pane_state: &'a mut pane_grid::State<MainView>) -> PaneGrid<'a, Message> {
        PaneGrid::new(pane_state, |_pane, content, _focus| {
            match &content.app_view {
                AppView::SidePanel(state) => state.view(&mut content.scroll),
                AppView::ImageQueue(state) => state.view(&mut content.scroll),
                AppView::ImageDisplay(state) => state.view(),
                AppView::TagInput(state) => state.view(&mut content.scroll, &mut content.text_input_state)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_resize(10, Message::Resized)
    }

    pub fn handle_keyboard(app: &mut App, event: keyboard::Event) {
        match event {
            keyboard::Event::KeyPressed { key_code, .. } => {
                // allow pulling up menu regardless of keyboard state
                if let keyboard::KeyCode::Escape = key_code {
                    app.app_state = AppState::Menu
                }

                if let KeyboardState::Tagging = app.keyboard_state {
                    match key_code {
                        keyboard::KeyCode::Left => {
                            let state = app.get_mut_state(app.image_queue).image_queue_mut();
                            match state.image_infos.prev(state.selected_image_index, |_| true) {
                                Some(x) => state.selected_image_index = x,
                                _ => ()
                            }
                        },
                        keyboard::KeyCode::Right => {
                            let state = app.get_mut_state(app.image_queue).image_queue_mut();
                            match state.image_infos.next(state.selected_image_index, |_| true) {
                                Some(x) => state.selected_image_index = x,
                                _ => ()
                            }
                        },
                        keyboard::KeyCode::Delete => {
                            let mut error: Option<io::Error> = None;
                            let mut path: Option<String> = None;

                            let state = app.get_mut_state(app.image_queue).image_queue_mut();
                            if state.selected_image_index < state.image_infos.len() {
                                if let Some(x) = state.image_infos.get(state.selected_image_index) {
                                    path = Some(x.path.clone());

                                    if let Err(e) = fs::remove_file(&x.path) {
                                        error = Some(e);
                                    }
                                }
                            }

                            match path {
                                Some(x) => {
                                    match error {
                                        Some(e) => app.log(format!("Error deleting {} : {}", x, e)),
                                        None => {
                                            state.delete_current();
                                            app.log(format!("Deleted {} successfully", x))
                                        }
                                    }
                                },
                                None => app.log("Error getting path of current file".to_string())
                            }
                        },
                        _ => ()
                    }
                }
            }
            keyboard::Event::CharacterReceived(character) => {
                match app.app_state {
                    AppState::Tagging => {
                        match app.keyboard_state {
                            KeyboardState::Tagging => { 
                                if character.is_alphabetic() {
                                    if !app.does_tag_exist(&character.to_string()) {
                                        app.keyboard_state = KeyboardState::None;
                                        let tag_input_content = MainView::new(AppView::TagInput(TagInputState { 
                                            tag_input_value: "".to_string(),
                                            tag: character
                                        }));

                                        let (pane, split) = app.pane_state
                                                               .split(pane_grid::Axis::Horizontal, 
                                                                    &app.image_display, tag_input_content)
                                                               .expect("Pane couldn't split");
                                        app.tag_input = Some(pane);
                                        app.pane_state.resize(&split, 0.9);
                                    }

                                    app.toggle_tag_on_current_image(&character);
                                } else {
                                    let state = app.get_mut_state(app.image_queue).image_queue_mut();
                                    match character {
                                        '[' => {
                                            if let Some(x) = state.image_infos
                                                                    .prev(state.selected_image_index,
                                                                          |x| x.tags.is_empty()) {
                                                state.selected_image_index = x;
                                            }
                                        },
                                        ']' => {
                                            if let Some(x) = state.image_infos
                                                                    .next(state.selected_image_index,
                                                                          |x| x.tags.is_empty()) {
                                                state.selected_image_index = x;
                                            }
                                        },
                                        '{' => {
                                            if let Some(x) = state.image_infos
                                                                    .prev(state.selected_image_index,
                                                                          |x| !x.tags.is_empty()) {
                                                state.selected_image_index = x;
                                            }
                                        },
                                        '}' => {
                                            if let Some(x) = state.image_infos
                                                                    .next(state.selected_image_index,
                                                                          |x| !x.tags.is_empty()) {
                                                state.selected_image_index = x;
                                            }
                                        },
                                        _ => ()
                                    }
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
}
