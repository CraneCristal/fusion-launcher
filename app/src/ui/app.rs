use crate::ui::localization::init_localization;
use crate::ui::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    init_localization();
    rsx! {
        style { {include_str!("style/compiled.css")} }
        Router::<Route> {}
    }
}
