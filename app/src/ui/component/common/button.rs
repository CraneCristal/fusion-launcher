use dioxus::prelude::*;

#[component]
pub fn Button(children: Element, class: Option<String>, on_click: EventHandler<MouseEvent>) -> Element {
    let class = class.unwrap_or(String::new());
    rsx! {
        link {
            rel: "stylesheet",
            href: "assets/styles/component/common/button.css"
        }
        button {
            class: "app-button {class}",
            onclick: move |event| on_click.call(event),
            { children }
        }
    }
}

#[component]
pub fn RadioButton(children: Element, class: Option<String>, on_click: EventHandler<MouseEvent>, is_selected: bool) -> Element {
    let mut class = class.unwrap_or(String::new());
    if is_selected {
       class += " selected";
    }

    rsx! {
        link {
            rel: "stylesheet",
            href: "assets/styles/component/common/button.css"
        }
        button {
            class: "app-button radio-button {class}",
            onclick: move |event| on_click.call(event),
            { children }
        }
    }
}
