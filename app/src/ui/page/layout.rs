use dioxus::prelude::*;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div { id: "layout-container",
            nav { id: "layout-nav" }
            div { id: "layout-children", { children } }
        }
    }
}
