pub mod app;
mod component;
mod page;
mod route;

use dioxus::desktop::{Config, WindowBuilder};

pub fn get_ui_config() -> Config {
    Config::new()
        .with_window(WindowBuilder::new().with_title("Fusion Launcher"))
        .with_menu(None)
}
