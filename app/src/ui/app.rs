use crate::ui::get_app_style;
use crate::ui::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let app_style = get_app_style();
    rsx! {
        style { "{app_style}" }
        Router::<Route> {}
    }
}
