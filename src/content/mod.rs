use iced::{ scrollable, Element, pane_grid };
use iced_native::{ text_input };

use crate::states::AppView;
use crate::states::Message;

#[derive(Debug)]
pub struct Content {
    pub app_view: AppView,
    scroll: scrollable::State,
    text_input_state: text_input::State
}

impl Content {
    pub fn new(app_view: AppView) -> Self {
        Content { 
            app_view: app_view,
            scroll: scrollable::State::new(),
            text_input_state: text_input::State::focused()
        }
    }

    pub fn view(&mut self, _pane: pane_grid::Pane) -> Element<Message> {
        match &self.app_view {
            AppView::SidePanel(state) => state.view(&mut self.scroll),
            AppView::ImageQueue(state) => state.view(&mut self.scroll),
            AppView::ImageDisplay(state) => state.view(&mut self.scroll),
            AppView::TagInput(state) => state.view(&mut self.scroll, &mut self.text_input_state)
        }
    }
}
