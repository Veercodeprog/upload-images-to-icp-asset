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
#[component]
fn AuthServiceProvider(children: Children) -> impl IntoView {
    let auth_service = Rc::new(RefCell::new(
        AuthService::new().expect("Failed to create AuthService"),
    ));
    provide_context(auth_service.clone());

    let canisters_signal = create_rw_signal(None);
    provide_context(canisters_signal);

    spawn_local({
        let auth_service = auth_service.clone();
        async move {
            match Canisters::new(auth_service).await {
                Ok(canisters_instance) => {
                    canisters_signal.set(Some(Rc::new(canisters_instance)));
                    log!("Canisters initialized successfully.");
                }
                Err(e) => log!("Failed to create Canisters: {:?}", e),
            }
        }
    });
    // Provide AuthService as a context
    children()
}
fn main() {
    // set up logging
    provide_meta_context();

    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <AuthServiceProvider>
                <App />
            </AuthServiceProvider>
        }
    })
}
