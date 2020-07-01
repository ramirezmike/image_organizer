use iced::{ Scrollable, scrollable, Length, Container, Element, Align };
use iced_native::{ text_input, TextInput };

use crate::enums::Message;
use crate::style;

#[derive(Debug)]
pub struct TagInputState {
    pub tag_input_value: String,
    pub tag: char
}

impl TagInputState {
    pub fn view<'a>(self: &Self,
                scroll: &'a mut scrollable::State,
                text_input_state: &'a mut text_input::State) -> Element<'a, Message> {

        let scrollable = Scrollable::new(scroll)
                        .align_items(Align::Start)
                        .push(TextInput::new(text_input_state, "Enter Tag Name", 
                                             &self.tag_input_value, Message::TextInputChanged)
                                        .on_submit(Message::TextInputSubmitted)
                                        .padding(10)
                                        .size(20));

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(self::style::Pane { })
            .center_x()
            .into()
    }
}
