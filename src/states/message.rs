use iced::{ pane_grid };
use iced_native::{ Event };
use crate::states::organize_mode::OrganizeMode;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(Event),
    TextInputChanged(String),
    TextInputSubmitted,
    Resized(pane_grid::ResizeEvent),
    SelectedOrganizeMode(OrganizeMode)
}
