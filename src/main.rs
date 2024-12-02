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
fn AuthServiceProvider(children: ChildrenFn) -> impl IntoView {
    let auth_service = Rc::new(RefCell::new(
        AuthService::new().expect("Failed to create AuthService"),
    ));
    provide_context(auth_service.clone());

    // Create the signal for Canisters and provide it
    let canisters_signal: RwSignal<Option<Rc<Canisters>>> = create_rw_signal(None);
    provide_context(canisters_signal.clone());

    // Initialize Canisters asynchronously
    spawn_local({
        let auth_service = auth_service.clone();
        let canisters_signal = canisters_signal.clone(); // Clone signal for async move
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

    view! {
        <Suspense fallback=move || {
            view! { <div>"Loading..."</div> }
        }>
            // Now you can safely call children()
            {children()}
        </Suspense>
    }
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
