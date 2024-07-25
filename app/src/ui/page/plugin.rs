use crate::ui::page::Layout;
use dioxus::prelude::*;

#[component]
pub fn PluginBrowser() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "assets/styles/page/plugins.css" }
        Layout {
            span { "Plugins" }
        }
    }
}
