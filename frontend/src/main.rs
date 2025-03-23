use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

use pages::common::Route;

use pages::crc32::CRC32;
use pages::home::Home;
use pages::patcher::Patcher;
use pages::portrait::Portrait;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(TitleBar)]
fn title_bar() -> Html {
    let close_window = Callback::from(|_| {
        spawn_local(async {
            let _ = invoke("window_close", JsValue::NULL).await;
        });
    });

    let minimize_window = Callback::from(|_| {
        spawn_local(async {
            let _ = invoke("window_minimize", JsValue::NULL).await;
        });
    });

    let start_drag = Callback::from(|_| {
        spawn_local(async {
            let _ = invoke("window_drag", JsValue::NULL).await;
        });
    });

    html! {
        <div
            class="titlebar"
            style="display: flex; align-items: center; cursor: default; z-index: 1000; position: relative;"
            onclick={start_drag}
            data-tauri-drag-region={true.to_string()}>

            <div class="titlebar-left" style="position: absolute; left: 0px;">
                <img
                    src="public/taskbar/icon.png"
                    alt="Icon"
                    style="width: 38px; height: 38px;"/>
            </div>

            <div style="display: flex; margin-left: auto;">
                <div class="titlebar-button" id="titlebar-minimize" onclick={minimize_window}>
                    <img
                        src="public/taskbar/minimize.svg"
                        alt="minimize"
                        style="width: 20px; height: 20px;"/>
                </div>
                <div class="titlebar-button" id="titlebar-close" onclick={close_window}>
                    <img
                        src="public/taskbar/close.svg"
                        alt="close"
                        style="width: 20px; height: 20px;"/>
                </div>
            </div>
        </div>

    }
}

#[function_component(App)]
pub fn app() -> Html { 
    html! {
        <>
            <TitleBar />
            <BrowserRouter>
                <main class="container">
                    <Switch<Route> render={switch} />
                </main>
            </BrowserRouter>
        </>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CRC32 => html! { <CRC32 /> },
        Route::Patcher => html! { <Patcher /> },
        Route::Portrait => html! { <Portrait /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
