use crate::ui::page::Layout;
use dioxus::prelude::*;

#[component]
pub fn PluginsPage() -> Element {
    rsx! {
        style { {include_str!("style/plugin.css")} }
        Layout {
            span { "Plugins" }
        }
    }
}
