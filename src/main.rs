mod app;
mod components;
mod data;
mod theme;

use app::App;
use leptos::prelude::*;

pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
