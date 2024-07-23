use dioxus::prelude::*;
use crate::ui::page::layout::Layout;

#[component]
pub fn GameLib() -> Element {
    rsx! {
        link {rel: "stylesheet", href: "assets/styles/page/game_lib.css"}
        Layout {
            span {"Game Lib"}
        }
    }
}