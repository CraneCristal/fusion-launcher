use crate::ui::localization::init_localization;
use crate::ui::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    init_localization();

    rsx! {
        link {rel: "stylesheet", href: "assets/styles/app.css"}
        Router::<Route> {}
    }
}
