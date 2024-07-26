mod error;
mod model;
mod ui;
mod utils;

use crate::error::Error;
use crate::ui::app::App;
use crate::ui::get_ui_config;
use crate::utils::dir::get_assets_dir;

fn main() -> Result<(), Error> {
    let assets_dir = get_assets_dir();
    println!("{}",get_assets_dir().to_str().unwrap());
    dioxus::desktop::launch::launch(App, vec![], get_ui_config());
    Ok(())
}
