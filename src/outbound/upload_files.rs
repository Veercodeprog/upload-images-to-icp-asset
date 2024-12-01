use crate::canister::asset_proxy::AssetProxy;
use crate::canister::asset_proxy::StoreArg;
use crate::canister::generated::asset_proxy;
use crate::canister::provision;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use anyhow::Error; // Ensure you have anyhow for error handling
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use gloo_file::File;
use leptos::logging::log;
use leptos::*;
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
// Define a type alias for clarity (optional)
type AssetKey = String;

pub async fn upload_files_from_input_event(
    event: Event,
    canisters: Rc<Canisters>,
) -> Result<Vec<String>, Error> {
    log!("Handling event: Canisters present.");

    let input = event
        .target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

    let mut asset_keys = Vec::new();

    if let Some(input) = input {
        if let Some(file_list) = input.files() {
            for i in 0..file_list.length() {
                if let Some(file) = file_list.get(i) {
                    let file = File::from(file);

                    let bytes = match read_as_bytes(&file).await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            log!("Failed to read file data: {:?}", e);
                            continue;
                        }
                    };

                    let store_arg = StoreArg {
                        key: format!("file-{}", file.name()),
                        content_type: file.raw_mime_type(),
                        content_encoding: "identity".to_string(),
                        content: ByteBuf::from(bytes),
                        sha256: None,
                        aliased: Some(false),
                    };
                    let asset_id = "6qg6m-4aaaa-aaaab-qacqq-cai";
                    let asset_principal =
                        Principal::from_text(asset_id).expect("Invalid principal");

                    // Call `store_asset` on the Canisters instance
                    match canisters.store_asset(asset_principal, store_arg).await {
                        Ok(()) => {
                            log!("File '{}' stored successfully", file.name());
                            asset_keys.push(format!("file-{}", file.name())); // Push the key
                        }
                        Err(e) => {
                            log!("Failed to store file '{}': {:?}", file.name(), e);
                        }
                    }
                }
            }
        }
    }

    Ok(asset_keys)
}
