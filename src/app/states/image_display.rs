use iced::{ Scrollable, scrollable, Length, Container, Element, Align, Text };

use crate::app::Message;
use crate::app::utils;

#[derive(Debug)]
pub struct ImageDisplayState {
    pub label: String,
    pub current_image_path: String,
}

impl ImageDisplayState {
    pub fn view<'a>(self: &'a Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {

        let scrollable = Scrollable::new(scroll)
                            .align_items(Align::Start)
                            .push(Text::new(self.label.to_string()).size(30)) 
                            .push(utils::image::load_image(self.current_image_path.clone()));

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}
