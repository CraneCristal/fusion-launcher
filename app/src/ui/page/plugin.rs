use crate::ui::layout::Layout;
use dioxus::prelude::*;

#[component]
pub fn PluginsPage() -> Element {
    rsx! {
        Layout {
            span { "Plugins" }
        }
    }
}
