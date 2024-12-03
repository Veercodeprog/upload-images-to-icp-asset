use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use leptos::*;
use leptos_meta::*;

mod state;
use leptos::logging::log;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use upload_images_to_icp_asset::App;
mod canister;

fn main() {
    // set up logging

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
