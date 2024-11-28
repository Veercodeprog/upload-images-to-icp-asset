use crate::canister::asset_proxy::store;
use crate::canister::asset_proxy::StoreArg;
use crate::canister::provision;
use gloo::file::futures::read_as_bytes;
use gloo_file::File;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::JsCast;

// use wasm_bindgen_futures::spawn_local;
use web_sys::{Event, HtmlInputElement};

pub fn upload_files_from_input_event(event: Event) {
    // Extract the HtmlInputElement from the event
    let input = event
        .target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

    if let Some(input) = input {
        if let Some(file_list) = input.files() {
            // Perform the upload asynchronously
            spawn_local(async move {
                // Iterate over each selected file
                for i in 0..file_list.length() {
                    if let Some(file) = file_list.get(i) {
                        let file = File::from(file);

                        // Read the file data
                        let bytes = match read_as_bytes(&file).await {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                log!("Failed to read file data: {:?}", e);
                                continue;
                            }
                        };

                        // Create the StoreArg
                        let store_arg = StoreArg {
                            key: format!("file-{}", file.name()),
                            content_type: file.raw_mime_type(),
                            content_encoding: "identity".to_string(),
                            content: bytes,
                            sha256: None, // Compute SHA-256 if necessary
                            aliased: Some(false),
                        };

                        // Upload the file using the canister's `store` method
                        // let client = AssetProxy::from(AssetProxy::canister_id().into());
                        match store(store_arg).await {
                            Ok(_) => {
                                log!("File '{}' stored successfully", file.name());
                                // Optionally, provide user feedback here
                            }
                            Err(e) => {
                                log!("Failed to store file '{}': {:?}", file.name(), e);
                                // Optionally, handle the error and provide user feedback
                            }
                        }
                    }
                }
            });
        }
    }
}
