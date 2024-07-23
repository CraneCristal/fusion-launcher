use crate::ui::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        link {rel: "stylesheet", href: "assets/styles/app.css"}
        Router::<Route> {}
    }
}
