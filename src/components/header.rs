use crate::state::{
    auth::AuthService,
    auth_actions::{create_login_action, create_logout_action},
};
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use std::cell::RefCell;
use std::rc::Rc;
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="flex fixed z-50 justify-between items-center px-8 w-full h-20 font-light shadow-sm transition-all bg-white/90 backdrop-blur-md">
            // Logo Section
            <div class="flex items-center space-x-2">
                <a href="/">
                    <img src="/public/img/fueldao.svg" alt="Fuel DAO Logo" class="h-8" />
                </a>
            </div>

            // Collections and Profile
            <div class="hidden absolute right-8 gap-8 items-center lg:flex z-[1]">
                <a href="/collections">

                    <span class="font-medium text-black">"Collections"</span>
                </a>
                <UserPrincipal />
            // <button class="p-2 text-white bg-black rounded-full">
            // <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            // <path stroke-linecap="round" stroke-linejoin="round" d="M12 14l9-5-9-5-9 5 9 5zm0 7l9-5-9-5-9 5 9 5z"/>
            // </svg>
            // </button>
            </div>
        </div>
    }
}

#[component]
fn UserPrincipal() -> impl IntoView {
    // Obtain AuthService from the context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    // Reactive signal for authentication state
    let is_authenticated = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| auth_service.borrow().is_authenticated()
    });

    // Reactive signal for principal
    let principal = create_memo({
        let auth_service = Rc::clone(&auth_service);
        move |_| {
            if is_authenticated() {
                auth_service.borrow().get_principal().ok()
            } else {
                None
            }
        }
    });

    // Use the reusable actions from auth_actions.rs
    let handle_login = create_login_action(Rc::clone(&auth_service));
    let handle_logout = create_logout_action(Rc::clone(&auth_service));
    view! {
        <Show
            when=move || is_authenticated()
            fallback=move || {
                view! {
                    <a
                        // on:click=move |_| handle_login.dispatch(())
                        href="/login"
                        class="p-2 text-white bg-black rounded-full"
                        target="_blank"
                    >

                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            class="w-4 h-4"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                            ></path>
                        </svg>
                    </a>
                }
            }
        >
            // <div>{"user: "}{move || principal().map(|p| p.to_text()).unwrap_or_default()}</div>

            <div class="flex items-center space-x-2">

                <a
                    // on:click=move |_| handle_logout.dispatch(())
                    href="/login"
                    class="flex justify-center items-center w-10 h-10 text-xl font-light text-white uppercase bg-black rounded-full select-none"
                    target="_blank"
                >

                    U
                </a>
            </div>
        </Show>
    }
}
