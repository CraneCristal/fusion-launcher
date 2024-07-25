mod error;
mod model;
mod ui;
mod utils;

use crate::error::Error;
use crate::ui::app::App;
use crate::ui::get_ui_config;

fn main() -> Result<(), Error> {
    dioxus::desktop::launch::launch(App, vec![], get_ui_config());
    Ok(())
}
