use crate::ui::component::common::radio_button::RadioButton;
use crate::utils::dir::get_assets_dir;
use dioxus::prelude::*;
use dioxus_sdk::i18n::use_i18;
use dioxus_sdk::translate;

#[component]
pub fn Layout(children: Element) -> Element {
    let assets_dir = get_assets_dir();
    let assets_dir_str = assets_dir.to_str().unwrap();

    let current_route = router().current_route_string();
    let i18 = use_i18();
    
    rsx! {
        div { id: "layout-container",
            nav { id: "layout-nav",
                RadioButton {
                    on_click: move |_event| {
                        router().push("/");
                    },
                    is_selected: current_route == "/",

                    img {
                        src: format!("{}/images/icons/home.svg", assets_dir_str),
                        alt: "home-icon"
                    }
                    {translate!(i18, "messages.home")}
                }
                RadioButton {
                    on_click: move |_event| {
                        router().push("/game-lib");
                    },
                    is_selected: current_route == "/game-lib",

                    img {
                        src: format!("{}/images/icons/library.svg", assets_dir_str),
                        alt: "game-lib-icon"
                    }
                    {translate!(i18, "messages.games")}
                }
                RadioButton {
                    on_click: move |_event| {
                        router().push("/plugins");
                    },
                    is_selected: current_route == "/plugins",

                    img {
                        src: format!("{}/images/icons/plugins.svg", assets_dir_str),
                        alt: "plugins-icon"
                    }
                    {translate!(i18, "messages.plugins")}
                }
                RadioButton {
                    on_click: move |_event| {},
                    is_selected: current_route == "/settings",
                    class: "settings-button",

                    img {
                        src: format!("{}/images/icons/settings.svg", assets_dir_str),
                        alt: "settings-icon"
                    }
                    {translate!(i18, "messages.settings")}
                }
            }
            div { id: "layout-children", { children } }
        }
    }
}
