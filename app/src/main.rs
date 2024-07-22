mod ui;

use crate::ui::{get_config, App};

fn main() {
    dioxus::desktop::launch::launch(App, vec![], get_config());
}
