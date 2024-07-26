use crate::ui::page::game_lib::GameLibPage;
use crate::ui::page::home::HomePage;
use crate::ui::page::plugin::PluginsPage;
use dioxus::prelude::*;

#[derive(Routable, Clone, Debug)]
pub enum Route {
    #[route("/")]
    HomePage {},

    #[route("/game-lib")]
    GameLibPage {},

    #[route("/plugins")]
    PluginsPage {},
}
