use crate::ui::layout::Layout;
use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Layout { 
            span { "Home" }
        }
    }
}
