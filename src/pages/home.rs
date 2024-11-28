use crate::outbound::upload_files;
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use gloo_file::Blob;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement}; // Import the upload function

/// Define the CarCollection struct
#[derive(Clone)]
pub struct CarCollection {
    pub id: u64,
    pub name: String,
    pub model: String,
    pub logo: String,           // Asset reference (e.g., URL or asset ID)
    pub images: Vec<String>,    // List of asset references
    pub documents: Vec<String>, // List of asset references
    pub owner: Principal,
    pub approved: bool,
}

// Implement Default manually due to Principal not implementing Default
impl Default for CarCollection {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            model: String::new(),
            logo: String::new(),
            images: Vec::new(),
            documents: Vec::new(),
            owner: Principal::anonymous(),
            approved: false,
        }
    }
}

#[component]
pub fn Home() -> impl IntoView {
    // Reactive state for CarCollection
    let (collection, set_collection) = create_signal(CarCollection::default());

    // Error message signal
    let (error_message, set_error_message) = create_signal(String::new());

    // Uploading state signals (use create_rw_signal)
    let uploading = create_rw_signal(false);
    let uploading_progress = create_rw_signal(0);

    // Handler for file selection (upload)
    let on_select = {
        let set_collection = set_collection.clone();
        let uploading = uploading.clone();
        let uploading_progress = uploading_progress.clone();
        let error_message = set_error_message.clone();

        move |event: Event, field: &'static str| {
            // Reset error message
            error_message.set(String::new());

            // Set uploading state
            uploading.set(true);
            uploading_progress.set(0);

            // Clone variables for async task
            let set_collection = set_collection.clone();
            let uploading = uploading.clone();
            let uploading_progress = uploading_progress.clone();
            let error_message = error_message.clone();

            // Call the upload_files_from_input_event function
            spawn_local(async move {
                match upload_files_from_input_event(event.clone()).await {
                    Ok(asset_keys) => {
                        // Update the collection state with the uploaded asset keys
                        if field == "logo" {
                            if let Some(asset_key) = asset_keys.first() {
                                set_collection.update(|c| {
                                    c.logo = asset_key.clone();
                                });
                            }
                        } else if field == "images" {
                            set_collection.update(|c| {
                                c.images.extend(asset_keys.clone());
                            });
                        } else if field == "documents" {
                            set_collection.update(|c| {
                                c.documents.extend(asset_keys.clone());
                            });
                        }
                        uploading_progress.set(100);
                    }
                    Err(e) => {
                        log::error!("Upload failed: {:?}", e);
                        error_message.set(format!("Upload failed: {:?}", e));
                    }
                }
                uploading.set(false);
                // Clear the file input value
                if let Some(input) = event
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                {
                    input.set_value("");
                }
            });
        }
    };

    // Remove image handler
    let remove_image = {
        let set_collection = set_collection.clone();
        move |image_path: String| {
            set_collection.update(move |c| {
                c.images.retain(|p| p != &image_path);
            });
        }
    };

    // Remove document handler
    let remove_document = {
        let set_collection = set_collection.clone();
        move |doc_path: String| {
            set_collection.update(move |c| {
                c.documents.retain(|p| p != &doc_path);
            });
        }
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>

            <div class="container">
                <h1>"Car Collection Form"</h1>

                <form class="flex flex-col gap-4">

                    // ID Field
                    <label class="block">
                        <span class="text-sm font-medium leading-6 text-gray-900">"ID:"</span>
                        <input
                            type="number"
                            value=move || collection().id.to_string()
                            on:input=move |e| {
                                let value = event_target_value(&e);
                                if let Ok(id) = value.parse::<u64>() {
                                    set_collection.update(|c| c.id = id);
                                }
                            }
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                            placeholder="Enter the car collection ID"
                        />
                    </label>

                    // Name Field
                    <label class="block">
                        <span class="text-sm font-medium leading-6 text-gray-900">"Name:"</span>
                        <input
                            type="text"
                            value=move || collection().name.clone()
                            on:input=move |e| {
                                let value = event_target_value(&e);
                                set_collection.update(|c| c.name = value);
                            }
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                            placeholder="Enter the car collection name"
                        />
                    </label>

                    // Model Field
                    <label class="block">
                        <span class="text-sm font-medium leading-6 text-gray-900">"Model:"</span>
                        <input
                            type="text"
                            value=move || collection().model.clone()
                            on:input=move |e| {
                                let value = event_target_value(&e);
                                set_collection.update(|c| c.model = value);
                            }
                            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm"
                            placeholder="Enter the car model"
                        />
                    </label>

                    // Logo Upload
                    <div class="mt-4 flex flex-col gap-2">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                                <div>
                                    {move || {
                                        if uploading.get() {
                                            format!("Uploading... ({}%)", uploading_progress.get())
                                        } else if !collection().logo.is_empty() {
                                            "Change logo".to_string()
                                        } else {
                                            "Upload logo".to_string()
                                        }
                                    }}
                                </div>
                            </div>

                            <input
                                on:change=move |e| (on_select)(e, "logo")
                                type="file"
                                accept="image/*"
                                class="sr-only"
                            />
                        </label>
                    </div>

                    // Display Logo
                    <Show when=move || !collection().logo.is_empty() fallback=|| ()>
                        <div class="mt-2 h-[14rem] w-[14rem] p-2 border rounded relative">
                            <button
                                on:click=move |_| {
                                    set_collection.update(|c| c.logo.clear());
                                }
                                class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                aria-label="Remove logo"
                            >
                                "X"
                            </button>
                            <img
                                src=collection().logo.clone()
                                alt="Logo"
                                class="h-full w-full rounded-md object-contain"
                            />
                        </div>
                    </Show>

                    // Images Upload and Display
                    <span class="text-sm font-medium leading-6 text-gray-900">"Images:"</span>
                    <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                        <For
                            each=move || collection().images.clone()
                            key=|path| path.clone()
                            let:path
                        >
                            move |path| {
                                let remove_image = remove_image.clone();
                                let path_clone = path.clone();
                                view! {
                                    <div class="p-1 shrink-0 border rounded-md w-52 h-52 relative">
                                        <button
                                            on:click=move |_| remove_image(path_clone.clone())
                                            class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                            aria-label=move || {
                                                format!("Remove image {}", path_clone.clone())
                                            }
                                        >
                                            "X"
                                        </button>
                                        <img
                                            src=path_clone.clone()
                                            class="h-full w-full rounded-md object-contain"
                                            alt=move || format!("Image {}", path_clone)
                                        />
                                    </div>
                                }
                            }
                        </For>
                        <Show when=move || collection().images.is_empty()>
                            <div class="flex flex-1 text-sm items-center justify-center">
                                "No images added yet"
                            </div>
                        </Show>
                    </div>

                    <div class="mt-4 flex flex-col gap-2">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                                <div>
                                    {move || {
                                        if uploading.get() {
                                            format!("Uploading... ({}%)", uploading_progress.get())
                                        } else {
                                            "Upload images".to_string()
                                        }
                                    }}
                                </div>
                            </div>

                            <input
                                on:change=move |e| (on_select)(e, "images")
                                type="file"
                                accept="image/*"
                                multiple=true
                                class="sr-only"
                            />
                        </label>
                    </div>

                    // Documents Upload and Display
                    <span class="text-sm font-medium leading-6 text-gray-900">"Documents:"</span>
                    <div class="h-[14rem] border rounded p-2 items-center w-full overflow-hidden overflow-x-auto flex gap-2">
                        <For
                            each=move || collection().documents.clone()
                            key=|path| path.clone()
                            let:path
                        >
                            move |path| {
                                let remove_document = remove_document.clone();
                                let path_clone = path.clone();
                                view! {
                                    <div class="p-1 shrink-0 border rounded-md w-52 h-52 relative">
                                        <button
                                            on:click=move |_| remove_document(path_clone.clone())
                                            class="bg-white rounded-full flex items-center justify-center w-4 h-4 absolute top-2 right-2"
                                            aria-label=move || {
                                                format!("Remove document {}", path_clone.clone())
                                            }
                                        >
                                            "X"
                                        </button>
                                        <div class="h-full w-full flex items-center justify-center">
                                            <a
                                                href=path_clone.clone()
                                                class="text-blue-500 underline"
                                                target="_blank"
                                            >
                                                {move || format!("Document {}", path_clone)}
                                            </a>
                                        </div>
                                    </div>
                                }
                            }
                        </For>
                        <Show when=move || collection().documents.is_empty()>
                            <div class="flex flex-1 text-sm items-center justify-center">
                                "No documents added yet"
                            </div>
                        </Show>
                    </div>

                    <div class="mt-4 flex flex-col gap-2">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="bg-primary shadow-md rounded-full flex items-center gap-2 font-semibold py-2 px-6 hover:bg-green-600 active:bg-green-500 transition-colors cursor-pointer text-nowrap text-sm text-white">
                                <div>
                                    {move || {
                                        if uploading.get() {
                                            format!("Uploading... ({}%)", uploading_progress.get())
                                        } else {
                                            "Upload documents".to_string()
                                        }
                                    }}
                                </div>
                            </div>

                            <input
                                on:change=move |e| (on_select)(e, "documents")
                                type="file"
                                accept="application/pdf"
                                multiple=true
                                class="sr-only"
                            />
                        </label>
                    </div>

                    // Approved Checkbox
                    <label class="flex items-center mt-4">
                        <input
                            type="checkbox"
                            checked=move || collection().approved
                            on:change=move |e| {
                                let checked = event_target_checked(&e);
                                set_collection.update(|c| c.approved = checked);
                            }
                            class="form-checkbox h-4 w-4 text-primary transition duration-150 ease-in-out"
                        />
                        <span class="ml-2 text-sm leading-5 text-gray-900">"Approved"</span>
                    </label>

                    // Error Message
                    <Show when=move || !error_message.get().is_empty()>
                        <div class="text-red-500 text-sm mt-2">{error_message.get()}</div>
                    </Show>

                    // Submit Button
                    <button
                        type="button"
                        on:click=move |_| {}
                        class="mt-4 bg-blue-500 hover:bg-blue-600 text-white font-semibold py-2 px-4 rounded"
                    >
                        "Submit"
                    </button>

                </form>
            </div>
        </ErrorBoundary>
    }
}