pub mod app;
mod component;
mod localization;
mod page;
mod route;
mod layout;

use crate::utils::dir::get_assets_dir;
use dioxus::desktop::{Config, WindowBuilder};

pub fn get_ui_config() -> Config {
    let css_template = r#"
            <style>
                @font-face {
                    font-family: "Mont Blanc";
                    src: url({{mont_blanc_regular}}) format("opentype");
                    font-weight: normal;
                    font-style: normal;
                }

                @font-face {
                    font-family: "Mont Blanc";
                    src: url({{mont_blanc_bold}}) format("opentype");
                    font-weight: bold;
                    font-style: normal;
                }

                @font-face {
                    font-family: "Mont Blanc";
                    src: url({{mont_blanc_semibold}}) format("opentype");
                    font-weight: 600;
                    font-style: normal;
                }
            </style>
            "#;

    let assets_dir = get_assets_dir();
    let assets_dir_str = assets_dir.to_str().unwrap();
    let css = css_template
        .replace(
            "{{mont_blanc_regular}}",
            format!("{}/fonts/mont_blanc_regular.otf", assets_dir_str).as_str(),
        )
        .replace(
            "{{mont_blanc_bold}}",
            format!("{}/fonts/mont_blanc_bold.otf", assets_dir_str).as_str(),
        )
        .replace(
            "{{mont_blanc_semibold}}",
            format!("{}/fonts/mont_blanc_semibold.otf", assets_dir_str).as_str(),
        );

    Config::new()
        .with_window(WindowBuilder::new().with_title("Fusion Launcher"))
        .with_menu(None)
        .with_custom_head(css)
}
