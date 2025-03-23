use wasm_bindgen_futures::spawn_local;

use gloo::file::futures::read_as_array_buffer;
use gloo::file::Blob;

use web_sys::{Event, HtmlInputElement};

use yew::prelude::*;
use yew_router::prelude::*;

use tauri_sys;

use serde::{Deserialize, Serialize};

use super::common::Route;

#[derive(Serialize, Deserialize)]
struct CRCArgs {
    filedata: Vec<u8>,
}

#[function_component(CRC32)]
pub fn crc32_page() -> Html {
    let checksum_data = use_state(|| String::new());

    // Clone the state handle to use inside the closure
    let checksum_data_clone = checksum_data.clone();

    // TODO: Make file selection open from backend since it's faster
    let on_file_upload = Callback::from(move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        if let Some(file_list) = input.files() {
            for i in 0..file_list.length() {
                let file = file_list.item(i).unwrap();
                web_sys::console::log_1(&format!("Selected file: {}", file.name()).into());

                let checksum_data = checksum_data_clone.clone();

                spawn_local(async move {
                    let blob = Blob::from(file);
                    match read_as_array_buffer(&blob).await {
                        Ok(buffer) => {
                            let array = js_sys::Uint8Array::new(&buffer);
                            let buffer_vec: Vec<u8> = array.to_vec();

                            let args = CRCArgs { filedata: buffer_vec };

                            let result = tauri_sys::core::invoke::<u32>("calculate_crc32", &args).await;

                            web_sys::console::log_1(&format!("CRC32 Result: {}", result).into());
                            checksum_data.set(result.to_string());
                        }
                        Err(err) => {
                            web_sys::console::log_1(&format!("Failed to read file: {}", err).into());
                        }
                    }
                });
            }
        }
    });

    let on_back_button_click = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.replace(&Route::Home);
        })
    };

    html! {
        <>
            <h1>{ "CRC32 Checksum Calculator" }</h1>
            <div class="crc32-resultbox">
                <p> <strong> { "Calculated Checksum: " } </strong> { format!("{}", *checksum_data) } </p>
            </div>
            <br/>
            <input
                id="file-upload"
                type="file"
                accept="*"
                multiple={false}
                onchange={on_file_upload}/>
            <button class="backbutton" onclick={on_back_button_click}>{"Back"}</button>
        </>
    }
}
