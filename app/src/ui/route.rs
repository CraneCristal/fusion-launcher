use crate::ui::page::home::Home;
use dioxus::prelude::*;

#[derive(Routable,Clone, Debug)]
pub enum Route {
    #[route("/")]
    Home {}
}