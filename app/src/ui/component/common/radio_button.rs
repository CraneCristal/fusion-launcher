use dioxus::prelude::*;

#[component]
pub fn RadioButton(children: Element, class: Option<String>, on_click: EventHandler<MouseEvent>, is_selected: bool) -> Element {
    let mut class = class.unwrap_or(String::new());
    if is_selected {
        class += " selected";
    }

    rsx! {
        button {
            class: "radio-button {class}",
            onclick: move |event| on_click.call(event),
            { children }
        }
    }
}