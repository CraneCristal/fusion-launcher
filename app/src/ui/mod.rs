pub mod app;
mod component;
mod page;
mod route;

use dioxus::desktop::{Config, WindowBuilder};

const APP_STYLE: &str = include_str!("../../assets/styles/compiled.css");

pub fn get_app_style() -> String {
    APP_STYLE.to_string()
}

pub fn get_app_config() -> Config {
    Config::new()
        .with_window(WindowBuilder::new().with_title("Fusion Launcher"))
        .with_menu(None)
}
