use crate::models::*;

#[derive(Debug)]
pub enum AppView {
    SidePanel(SidePanelState),
    ImageQueue(ImageQueueState),
    ImageDisplay(ImageDisplayState),
    TagInput(TagInputState),
}

impl AppView {
    pub fn side_panel(self: &Self) -> &SidePanelState {
        match self {
            AppView::SidePanel(x) => x,
            _ => panic!("Incorrect variant requested")
        }
    }

    pub fn image_queue(self: &Self) -> &ImageQueueState {
        match self {
            AppView::ImageQueue(x) => x,
            _ => panic!("Incorrect variant requested")
        }
    }

    pub fn tag_input(self: &Self) -> &TagInputState {
        match self {
            AppView::TagInput(x) => x,
            _ => panic!("Incorrect variant requested")
        }
    }

    pub fn image_display_mut(self: &mut Self) -> &mut ImageDisplayState {
        match self {
            AppView::ImageDisplay(x) => x,
            _ => panic!("Incorrect variant requested")
        }
    }

    pub fn image_queue_mut(self: &mut Self) -> &mut ImageQueueState {
        match self {
            AppView::ImageQueue(x) => x,
            _ => panic!("Incorrect variant requested")
        }
    }
}
