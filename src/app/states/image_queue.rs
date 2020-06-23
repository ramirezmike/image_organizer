use iced::{ Scrollable, scrollable, Length, 
            Column, Row, Container, Element, Align, Text };
use std::{ cmp };

use crate::app::Message;
use crate::app::utils;
use crate::app::style;

#[derive(Debug)]
pub struct ImageQueueState {
    pub selected_image_index: usize,
    pub image_paths: Vec::<String>,
}
impl ImageQueueState {
    pub fn new(path: &str) -> ImageQueueState {
        let mut image_paths = utils::file_io::get_directory_list(path).unwrap_or(Vec::<String>::new());
        image_paths.sort_unstable();

        ImageQueueState { 
            selected_image_index: 0,
            image_paths: image_paths
        }
    }

    pub fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let mut row = Row::<'_, Message>::new();

        let start = if self.selected_image_index < 3 { 0 } else { self.selected_image_index - 2 };
        let end = cmp::min(self.image_paths.len(), self.selected_image_index + 10);
        let mut item_index: usize = start;

        // in lieu of horizontal scrolling, show a shifting window of directory
        row = self.image_paths[start..end]
                      .iter()
                      .fold(row, |r, image_path| {
                          let text = Text::new(image_path.to_string());
                          let column = Column::<'_, Message>::new().push(text);
                          let style = style::ImageQueueItem {
                              is_selected: item_index == self.selected_image_index
                          };

                          item_index = item_index + 1;

                          r.push(Container::new(column)
                           .width(Length::Shrink)
                           .height(Length::Fill)
                           .style(style))
                      });

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
