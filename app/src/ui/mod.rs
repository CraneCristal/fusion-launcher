mod style;

use crate::ui::style::get_app_style;
use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        style { "{get_app_style()}" }
    }
}

pub fn get_config() -> Config {
    Config::new()
        .with_window(
            WindowBuilder::new()
            .with_title("Fusion Launcher")
        )
        .with_menu(None)
}
