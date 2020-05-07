use orbtk::prelude::*;

fn main() {
    println!("Hello, world!");

    Application::new()
        .window(|ctx| {
            Window::create().title("image_organizer")
                            .position((100.0, 100.0))
                            .size(420.0, 730.0)
                            .child(TextBlock::create().text("Hello World").build(ctx))
                            .build(ctx)
        }).run();
}
