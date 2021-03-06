use iced::{ Scrollable, scrollable, Length, 
            Column, Row, Container, Element, Align, Text };
use std::{ cmp, collections::HashMap };

use crate::states::Message;
use crate::util;
use crate::style;

#[derive(Debug)]
pub struct ImageQueueState {
    pub selected_image_index: usize,
    pub image_infos: Vec::<ImageInfo>,
}

#[derive(Debug)]
pub struct ImageInfo {
    pub path: String,
    pub tags: HashMap<char, ()>
}

impl ImageQueueState {
    pub fn new() -> ImageQueueState {
        let mut image_paths = util::file_io::get_directory_list(&".")
                                .unwrap_or(Vec::<String>::new());
        image_paths.sort_unstable();

        ImageQueueState { 
            selected_image_index: 0,
            image_infos: image_paths.iter()
                                    .map(|x| 
                                        ImageInfo { 
                                            path: x[2..].to_string(), // ignore "./" in path
                                            tags: HashMap::<char, ()>::new() 
                                        }
                                    )
                                    .collect()
        }
    }

    pub fn delete_current(self: &mut Self) {
        if self.selected_image_index < self.image_infos.len() {
            self.image_infos.remove(self.selected_image_index);
            if self.selected_image_index == self.image_infos.len() {
                self.selected_image_index = self.selected_image_index - 1;
            } 
        }
    }

    pub fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {
        let mut row = Row::<'_, Message>::new();

        let start = if self.selected_image_index < 3 { 0 } else { self.selected_image_index - 2 };
        let end = cmp::min(self.image_infos.len(), self.selected_image_index + 10);
        let mut item_index: usize = start;

        // in lieu of horizontal scrolling, show a shifting window of directory
        row = self.image_infos[start..end]
                      .iter()
                      .fold(row, |r, image_info| {
                          let text = Text::new(image_info.path.to_string());
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
