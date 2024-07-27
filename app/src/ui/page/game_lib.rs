use dioxus::prelude::*;
use crate::ui::layout::Layout;

#[component]
pub fn GameLibPage() -> Element {
    rsx! {
        Layout { 
            span { "Game Lib" }
        }
    }
}