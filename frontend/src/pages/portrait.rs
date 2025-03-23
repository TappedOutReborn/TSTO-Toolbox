use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::prelude::*;

use super::common::Route;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(module = "@tauri-apps/api/event", js_name = "listen", catch)]
    async fn listen(event: &str, handler: JsValue) -> Result<JsValue, JsValue>;
}

#[function_component(Portrait)]
pub fn portrait_page() -> Html {

    let on_back_button_click = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.replace(&Route::Home);
        })
    };

    html! {
        <>
            <h1>{ "Portrait" }</h1>

            <button class="backbutton" onclick={on_back_button_click}>{"Back"}</button>
        </>
    }
}
