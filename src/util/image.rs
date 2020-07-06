use iced::{ Length, Image, Container };

pub fn load_image<'a, T>(src: String) -> Container<'a, T> {
    Container::new(Image::new(src))
        .height(Length::Shrink)
        .center_x()
}
