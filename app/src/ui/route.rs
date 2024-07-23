use crate::ui::page::game_lib::GameLib;
use crate::ui::page::home::Home;
use crate::ui::page::plugin::PluginBrowser;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/game-lib")]
    GameLib {},

    #[route("/plugins")]
    PluginBrowser {},
}
