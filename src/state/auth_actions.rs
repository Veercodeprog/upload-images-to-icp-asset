// src/state/auth/auth_actions.rs

use crate::state::auth::AuthService;
use leptos::*;
use leptos_dom::logging::{console_error, console_log};
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a login action.
pub fn create_login_action(auth_service: Rc<RefCell<AuthService>>) -> Action<(), ()> {
    create_action(move |_: &()| {
        let auth_service = Rc::clone(&auth_service);
        async move {
            match auth_service.borrow_mut().login().await {
                Ok(_) => {
                    window().location().reload().unwrap();
                    console_log("Login successful.")
                }
                Err(e) => console_error(&format!("Login failed: {:?}", e)),
            }
        }
    })
}

/// Creates a logout action.
pub fn create_logout_action(auth_service: Rc<RefCell<AuthService>>) -> Action<(), ()> {
    create_action(move |_: &()| {
        let auth_service = Rc::clone(&auth_service);
        async move {
            match auth_service.borrow_mut().logout().await {
                Ok(_) => {
                    console_log("Logout successful.");
                    window().location().reload().unwrap();
                }
                Err(e) => console_error(&format!("Logout failed: {:?}", e)),
            }
        }
    })
}
