use crate::components::header::Header;
use crate::outbound::upload_files::upload_files_from_input_event;
use crate::state::auth::AuthService;
use crate::state::canisters::Canisters;
use candid::Principal;
use gloo::file::futures::read_as_bytes;
use gloo_file::Blob;
use leptos::logging::log;
use leptos::*;
use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
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
    //
    // let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
    //     .expect("Canisters signal not found in context");

    if let Some(canisters_signal) = use_context::<RwSignal<Option<Rc<Canisters>>>>() {
        let canisters_option = canisters_signal.get();
        match canisters_option {
            Some(canisters) => {
                log!("Canisters instance available");
                // Use canisters here
            }
            None => log!("Canisters instance not yet available"),
        }
    } else {
        log!("Canisters signal context not found");
    } // Get the current value of the signal
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

            spawn_local(async move {
                // let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
                //     .expect("Canisters signal not found in context");
                // if let Some(canisters_signal) = use_context::<RwSignal<Option<Rc<Canisters>>>>() {
                //     let canisters_option = canisters_signal.get();
                //     match canisters_option {
                //         Some(canisters) => {
                //             log!("Canisters instance available");
                //             // Use canisters here
                //         }
                //         None => log!("Canisters instance not yet available"),
                //     }
                // } else {
                //     log!("Canisters signal context not found");
                // } // Get the current value of the signal
                // let canisters_option = canisters_signal.clone();
                let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
                    .expect("Canisters signal not found in context");
                let canisters_option = canisters_signal.clone();
                match canisters_option.get() {
                    Some(canisters) => {
                        match upload_files_from_input_event(event.clone(), canisters).await {
                            Ok(asset_keys) => {
                                // Handle success
                                match field {
                                    "logo" => {
                                        if let Some(asset_key) = asset_keys.first() {
                                            set_collection.update(|c| {
                                                c.logo = asset_key.clone();
                                            });
                                        }
                                    }
                                    "images" => {
                                        set_collection.update(|c| {
                                            c.images.extend(asset_keys.clone());
                                        });
                                    }
                                    "documents" => {
                                        set_collection.update(|c| {
                                            c.documents.extend(asset_keys.clone());
                                        });
                                    }
                                    _ => {
                                        log::warn!("Unknown field: {}", field);
                                    }
                                }
                                uploading_progress.set(100);
                            }
                            Err(e) => {
                                // Handle error
                                log::error!("Upload failed: {:?}", e);
                                error_message.set(format!("Upload failed: {:?}", e));
                            }
                        }
                    }
                    None => {
                        log::error!("Canisters not available. Please log in.");
                        error_message.set("Canisters not available. Please log in.".to_string());
                    }
                }

                uploading.set(false);

                match event
                    .target()
                    .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                {
                    Some(input) => {
                        input.set_value("");
                    }
                    None => {
                        log::warn!("Could not clear input field");
                    }
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
                <Header />
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
                            class="block mt-1 w-full rounded-md border-gray-300 shadow-sm"
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
                            class="block mt-1 w-full rounded-md border-gray-300 shadow-sm"
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
                            class="block mt-1 w-full rounded-md border-gray-300 shadow-sm"
                            placeholder="Enter the car model"
                        />
                    </label>

                    // Logo Upload
                    <div class="flex flex-col gap-2 mt-4">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="flex gap-2 items-center py-2 px-6 text-sm font-semibold text-white rounded-full shadow-md transition-colors cursor-pointer hover:bg-green-600 active:bg-green-500 bg-primary text-nowrap">
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
                        <div class="relative p-2 mt-2 rounded border h-[14rem] w-[14rem]">
                            <button
                                on:click=move |_| {
                                    set_collection.update(|c| c.logo.clear());
                                }
                                class="flex absolute top-2 right-2 justify-center items-center w-4 h-4 bg-white rounded-full"
                                aria-label="Remove logo"
                            >
                                "X"
                            </button>
                            <img
                                src=collection().logo.clone()
                                alt="Logo"
                                class="object-contain w-full h-full rounded-md"
                            />
                        </div>
                    </Show>

                    // Images Upload and Display
                    <span class="text-sm font-medium leading-6 text-gray-900">"Images:"</span>
                    <div class="flex overflow-hidden overflow-x-auto gap-2 items-center p-2 w-full rounded border h-[14rem]">
                        <For
                            each=move || collection().images.clone()
                            key=|path| path.clone()
                            let:path
                        >
                            move |path|
                            {
                                let remove_image = remove_image.clone();
                                let path_clone = path.clone();
                                let path_clone1 = path.clone();
                                let path_clone2 = path.clone();
                                let path_clone3 = path.clone();
                                view! {
                                    <div class="relative p-1 w-52 h-52 rounded-md border shrink-0">
                                        <button
                                            on:click=move |_| remove_image(path_clone1.clone())
                                            class="flex absolute top-2 right-2 justify-center items-center w-4 h-4 bg-white rounded-full"
                                            aria-label=move || {
                                                format!("Remove image {}", path_clone2)
                                            }
                                        >
                                            "X"
                                        </button>
                                        <img
                                            src=path_clone3.clone()
                                            class="object-contain w-full h-full rounded-md"
                                            alt=move || format!("Image {}", path_clone3)
                                        />
                                    </div>
                                }
                            }
                        </For>
                        <Show when=move || collection().images.is_empty()>
                            <div class="flex flex-1 justify-center items-center text-sm">
                                "No images added yet"
                            </div>
                        </Show>
                    </div>

                    <div class="flex flex-col gap-2 mt-4">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="flex gap-2 items-center py-2 px-6 text-sm font-semibold text-white rounded-full shadow-md transition-colors cursor-pointer hover:bg-green-600 active:bg-green-500 bg-primary text-nowrap">
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
                    <div class="flex overflow-hidden overflow-x-auto gap-2 items-center p-2 w-full rounded border h-[14rem]">
                        <For
                            each=move || collection().documents.clone()
                            key=|path| path.clone()
                            let:path
                        >
                            move |path|
                            {
                                let remove_document = remove_document.clone();
                                let path_clone = path.clone();
                                let path_clone1 = path.clone();
                                let path_clone2 = path.clone();
                                view! {
                                    <div class="relative p-1 w-52 h-52 rounded-md border shrink-0">
                                        <button
                                            on:click=move |_| remove_document(path_clone.clone())
                                            class="flex absolute top-2 right-2 justify-center items-center w-4 h-4 bg-white rounded-full"
                                            aria-label=move || {
                                                format!("Remove document {}", path_clone2)
                                            }
                                        >
                                            "X"
                                        </button>
                                        <div class="flex justify-center items-center w-full h-full">
                                            <a
                                                href=path_clone1.clone()
                                                class="text-blue-500 underline"
                                                target="_blank"
                                            >
                                                {move || format!("Document {}", path_clone1)}
                                            </a>
                                        </div>
                                    </div>
                                }
                            }
                        </For>
                        <Show when=move || collection().documents.is_empty()>
                            <div class="flex flex-1 justify-center items-center text-sm">
                                "No documents added yet"
                            </div>
                        </Show>
                    </div>

                    <div class="flex flex-col gap-2 mt-4">
                        <label class=move || {
                            format!(
                                "w-min transition-opacity {}",
                                if uploading.get() { "pointer-events-none opacity-50" } else { "" },
                            )
                        }>
                            <div class="flex gap-2 items-center py-2 px-6 text-sm font-semibold text-white rounded-full shadow-md transition-colors cursor-pointer hover:bg-green-600 active:bg-green-500 bg-primary text-nowrap">
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
                            class="w-4 h-4 transition duration-150 ease-in-out form-checkbox text-primary"
                        />
                        <span class="ml-2 text-sm leading-5 text-gray-900">"Approved"</span>
                    </label>

                    // Error Message
                    <Show when=move || !error_message.get().is_empty()>
                        <div class="mt-2 text-sm text-red-500">{error_message.get()}</div>
                    </Show>

                    // Submit Button
                    <button
                        type="button"
                        on:click=move |_| {}
                        class="py-2 px-4 mt-4 font-semibold text-white bg-blue-500 rounded hover:bg-blue-600"
                    >
                        "Submit"
                    </button>

                </form>
            </div>
        </ErrorBoundary>
    }
}
