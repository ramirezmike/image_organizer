use iced::{ Text, Column, Row, Container, Radio };
use iced_native::{ keyboard };
use crate::app::App;
use crate::states::{ Message, OrganizeMode, AppState };

pub struct MenuView { }

impl MenuView {
    pub fn handle_keyboard(app: &mut App, event: keyboard::Event) {
        match event {
            keyboard::Event::KeyPressed { key_code, .. } => {
                match key_code {
                    keyboard::KeyCode::Escape => {
                        app.app_state = AppState::Tagging 
                    }
                    keyboard::KeyCode::Q => {
                        // TODO: exit more gracefully
                        assert!(1 == 0); // ¯\_(ツ)_/¯
                    }
                    keyboard::KeyCode::O => {
                        app.organize_mode.next();
                    }
                    keyboard::KeyCode::R => {
                        match app.run_organize_process() {
                            Err(e) => panic!("Error running process: {}", e),
                            _ => ()
                        }
                    }
                    keyboard::KeyCode::C => {
                        app.console_messages.clear();
                    }
                    _ => ()
                }
            },
            _ => ()
        }
    }

    pub fn view(app: &App) -> Column<Message> {
        let column = Column::<'_, Message>::new()
                        .push(Row::<'_, Message>::new()
                            .push(Container::new(Text::new("Q - Quit"))))
                        .push(Row::<'_, Message>::new()
                            .push(Container::new(Text::new("O - Organize Mode")))
                            .push(Radio::new(
                                    OrganizeMode::Copy, 
                                    "Copy", 
                                    Some(app.organize_mode), 
                                    Message::SelectedOrganizeMode))
                            .push(Radio::new(
                                    OrganizeMode::Move, 
                                    "Move", 
                                    Some(app.organize_mode), 
                                    Message::SelectedOrganizeMode))
                            .push(Radio::new(
                                    OrganizeMode::Link, 
                                    "Link", 
                                    Some(app.organize_mode), 
                                    Message::SelectedOrganizeMode))
                            )
                        .push(Row::<'_, Message>::new()
                            .push(Container::new(Text::new("R - Run Organize Process"))))
                        .push(Row::<'_, Message>::new()
                            .push(Container::new(Text::new("C - Clear Console"))))
                        .push(Row::<'_, Message>::new()
                            .push(Container::new(Text::new("Escape - Close Menu"))));

        app.console_messages.iter()
                            .fold(column, |acc, message| {
                                let text = Text::new(message);
                                let container = Container::new(text);
                                let row = Row::<'_, Message>::new()
                                              .push(container);

                                acc.push(row)
                            })
    }
}
