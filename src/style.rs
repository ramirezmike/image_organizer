use iced::{container, Background, Color};

const BACKGROUND: Color = Color::from_rgb(
    0x1F as f32 / 255.0,
    0x24 as f32 / 255.0,
    0x30 as f32 / 255.0,
);
const TEXT : Color = Color::from_rgb(
    0xCB as f32 / 255.0,
    0xCC as f32 / 255.0,
    0xC6 as f32 / 255.0,
);

pub struct MainWindow { }
impl container::StyleSheet for MainWindow {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(TEXT),
            background: Some(Background::Color(BACKGROUND)),
            border_width: 2,
            border_color: Color {
                a: 0.3,
                ..Color::BLACK
            },
            ..Default::default()
        }
    }
}

pub struct ImageQueue { }
impl container::StyleSheet for ImageQueue {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(TEXT),
            background: Some(Background::Color(BACKGROUND)),
            ..Default::default()
        }
    }
}

pub struct ImageQueueItem { }
impl container::StyleSheet for ImageQueueItem {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(TEXT),
            background: Some(Background::Color(BACKGROUND)),
            border_width: 2,
            border_color: Color {
                a: 0.3,
                ..Color::BLACK
            },
            ..Default::default()
        }
    }
}

pub struct Pane { }
impl container::StyleSheet for Pane {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(TEXT),
            background: Some(Background::Color(BACKGROUND)),
            ..Default::default()
        }
    }
}
