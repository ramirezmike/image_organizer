use std::env;
use iced::{ Settings, Application };

mod app;
mod style;
mod util;
mod content;
mod models;
mod lib_ext;
mod states;

fn main() {
    println!("Hello, world!");

    let font_config_key = "FONTCONFIG_FILE";
    let font_config_value = "/etc/fonts";
    match env::var(font_config_key) {
        Ok(_) => {
            println!("{} set.. (if you have font-issues, check this environment variable", font_config_key);
        }
        Err(_) => {
            println!("Issue with environment variable {}. Setting to {}", font_config_key, font_config_value);
            env::set_var(font_config_key, font_config_value);
        }
    }

    app::App::run(Settings::default()); 
}
