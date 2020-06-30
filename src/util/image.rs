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
        .width(Length::Fill)  // TODO: Not sure if it would be handled here or in resize
        .height(Length::Fill) // but it'd be good to make the images resize correctly when the window
                              // resizes so it doesn't introduce scrolling (maybe don't put this in a 
                              // scrollable container?)
    )
    .width(Length::Fill)
    .center_x()
}
