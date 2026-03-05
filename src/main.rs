mod i18n;
mod ui;

use dioxus::prelude::*;

fn main() {
    LaunchBuilder::web().launch(ui::app);
}