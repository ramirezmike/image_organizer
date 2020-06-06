use iced::{ Settings, Application };

mod app;

fn main() {
    println!("Hello, world!");
    app::App::run(Settings::default()); 
}
