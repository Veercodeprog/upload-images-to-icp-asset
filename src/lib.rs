use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod outbound;
// Modules
mod canister;
mod components;
mod consts;
mod pages;
mod state;
// mod stores;
// mod utils;
use std::cell::RefCell;
use std::rc::Rc;
// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use leptos::logging::log;

/// An app router which renders the homepage and handles 404's

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
                }
                Err(e) => log!("Failed to create Canisters: {:?}", e),
            }
        }
    });

    // Provide AuthService as a context
    children()
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <AuthServiceProvider>
            <Router>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/*" view=NotFound />
                </Routes>
            </Router>
        </AuthServiceProvider>
    }
}
