use iced::{ Length, Container, Column, Row, Element, Align, Text };

use crate::states::Message;
use crate::util;

#[derive(Debug)]
pub struct ImageDisplayState {
    pub root_path: String,
    pub current_image_path: String,
    pub current_image_tags: Option<Vec::<char>>,
}

impl ImageDisplayState {
    pub fn view<'a>(self: &'a Self) -> Element<'a, Message> {
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


        let column = Column::<'_, Message>::new()
                            .align_items(Align::Start)
                            .push(util::image::load_image(self.root_path.clone() + &self.current_image_path.clone()))
                            .push(tag_row);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .center_y()
            .center_x()
            .into()
    }
}
