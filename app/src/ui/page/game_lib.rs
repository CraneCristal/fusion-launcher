use dioxus::prelude::*;
use crate::ui::page::Layout;

#[component]
pub fn GameLibPage() -> Element {
    rsx! {
        style { {include_str!("style/game_lib.css")} }
        Layout { 
            span { "Game Lib" }
        }
    }
}