use crate::ui::page::Layout;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        link {rel: "stylesheet", href: "assets/styles/page/home.css"}
        Layout {
            span {"Home"}
        }
    }
}
