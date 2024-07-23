mod ui;

use crate::ui::app::App;
use crate::ui::get_app_config;

fn main() {
    dioxus::desktop::launch::launch(App, vec![], get_app_config());
}
