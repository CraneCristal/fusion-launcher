use crate::ui::component::common::button::RadioButton;
use dioxus::prelude::*;

#[component]
pub fn Layout(children: Element) -> Element {
    let current_route = router().current_route_string();

    rsx! {
        link { rel: "stylesheet", href: "assets/styles/page/layout.css" }
        div { id: "layout-container",
            nav { id: "layout-nav",
                RadioButton {
                    on_click: move |_event| {
                        router().push("/");
                    },
                    is_selected: current_route == "/",

                    img {
                        src: "assets/images/icons/home.svg",
                        alt: "home-icon"
                    }
                    "Home"
                }
                RadioButton {
                    on_click: move |_event| {
                        router().push("/game-lib");
                    },
                    is_selected: current_route == "/game-lib",

                    img {
                        src: "assets/images/icons/library.svg",
                        alt: "home-icon"
                    }
                    "Games"
                }
                RadioButton {
                    on_click: move |_event| {
                        router().push("/plugins");
                    },
                    is_selected: current_route == "/plugins",
                    
                    img {
                        src: "assets/images/icons/plugins.svg",
                        alt: "home-icon"
                    }
                    "Plugins"
                }
                RadioButton {
                    on_click: move |_event| {},
                    is_selected: current_route == "/settings",
                    class: "settings-button",
                    
                    img {
                        src: "assets/images/icons/settings.svg",
                        alt: "home-icon"
                    }
                    "Settings"
                }
            }
            div { id: "layout-children", { children } }
        }
    }
}
