mod ui;

use crate::ui::app::App;
use crate::ui::get_ui_config;

fn main() {
    dioxus::desktop::launch::launch(App, vec![], get_ui_config());
}
