use iced::{ Scrollable, scrollable, Length, 
            Row, Container, Element, Align, Text };
use std::{ collections::HashMap };

use crate::enums::Message;
use crate::style;

#[derive(Debug)]
pub struct SidePanelState { 
    pub label: String,
    pub tags: HashMap<String, String>
}


impl SidePanelState {
    pub fn view<'a>(self: &Self, scroll: &'a mut scrollable::State) -> Element<'a, Message> {

        let mut scrollable = Scrollable::new(scroll)
            .align_items(Align::Start)
            .push(Text::new(self.label.to_string()).size(30));

        for x in self.tags.iter() {
            let mut viewable_text = String::from(x.0);
            viewable_text.push_str(" - "); 
            viewable_text.push_str(x.1); 
            let text = Text::new(viewable_text);
            scrollable = scrollable.push(Row::<'_, Message>::new()
                                                    .push(Container::new(text)
                                                    .width(Length::Fill)
                                                    .height(Length::Shrink)));
        }
            
        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Pane { })
            .center_x()
            .into()
    }
}
