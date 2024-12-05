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
use sha2::{Digest, Sha256};
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
                    log!("Preparing to upload file: {}", file.name());

                    let bytes = match read_as_bytes(&file).await {
                        Ok(bytes) => {
                            log!("Read {} bytes from file: {}", bytes.len(), file.name());
                            bytes
                        }
                        Err(e) => {
                            log!("Failed to read file data for {}: {:?}", file.name(), e);
                            continue;
                        }
                    };
                    let computed_hash = Sha256::digest(&bytes);
                    let sha256 = Some(ByteBuf::from(computed_hash.to_vec()));
                    log!("Computed SHA-256 for {}: {:x}", file.name(), computed_hash);

                    let store_arg = StoreArg {
                        key: format!("/file-{}", file.name()),
                        content_type: file.raw_mime_type(),
                        content_encoding: "identity".to_string(),
                        content: ByteBuf::from(bytes),
                        sha256, // Provide the computed hash
                                // aliased: Some(false), // Explicitly set aliased
                    };
                    let asset_id = "zcs7y-5iaaa-aaaam-adxfq-cai";

                    let asset_principal =
                        Principal::from_text(asset_id).expect("Invalid principal");
                    log!("Uploading file: {}, Principal: {}", store_arg.key, asset_id);

                    log!("file name: {}  ", file.name());
                    // Call `store_asset` on the Canisters instance
                    match canisters
                        .store_asset(asset_principal, store_arg.clone())
                        .await
                    {
                        Ok(_) => {
                            log!("Successfully uploaded asset: {}", store_arg.key);
                            log!("https://{}.raw.icp0.io{}", asset_id, store_arg.key);
                            log!("https://{}.icp0.io{}", asset_id, store_arg.key);
                            asset_keys.push(store_arg.key);
                        }
                        Err(e) => {
                            log!("Failed to upload asset {}: {:?}", store_arg.key, e);
                        }
                    }
                }
            }
        }
    }

    Ok(asset_keys)
}
