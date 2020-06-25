use iced::{ Scrollable, scrollable, Length, Container, Column, Row, Element, Align, Text };

use crate::app::Message;
use crate::app::utils;

#[derive(Debug)]
pub struct ImageDisplayState {
    pub label: String,
    pub current_image_path: String,
    pub current_image_tags: Option<Vec::<char>>,
}

impl ImageDisplayState {
    pub fn view<'a>(self: &'a Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let mut tag_row = Row::<'_, Message>::new();
        match &self.current_image_tags {
            Some(tags) => tag_row = tags.iter()
                                        .fold(tag_row, |r, tag| {
                                            let column = Column::<'_, Message>::new()
                                                             .push(Text::new(tag.to_string()));
                                            r.push(Container::new(column)
                                             .width(Length::Shrink)
                                             .height(Length::Shrink))
                                        }),
            _ => ()
        }


        let scrollable = Scrollable::new(scroll)
                            .align_items(Align::Start)
                            .push(Text::new(self.label.to_string()).size(30)) 
                            .push(utils::image::load_image(self.current_image_path.clone()))
                            .push(tag_row);

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into()
    }
}
