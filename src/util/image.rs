use iced::{ Length, Image, Container };

pub fn load_image<'a, T>(src: String) -> Container<'a, T> {
    Container::new(
        // This implementation was based on the "tour" example
        // https://github.com/hecrj/iced/blob/master/examples/tour/src/main.rs
        // At the time, it said the following:
        //
        // This should go away once we unify resource loading on native platforms
        //
        if cfg!(target_arch = "wasm32") {
            Image::new(src)
        } else {
            Image::new(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), src))
        }
    )
    .height(Length::Shrink)
    .center_x()
}
