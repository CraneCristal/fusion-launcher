use crate::ui::page::Layout;
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        style { {include_str!("style/home.css")} }
        Layout { 
            span { "Home" }
        }
    }
}
