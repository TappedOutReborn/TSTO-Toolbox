use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" { 
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
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
            src="public/icon.png"
            alt="Icon"
            style="width: 38px; height: 38px;"/>
    </div>

    <div style="display: flex; margin-left: auto;">
        <div class="titlebar-button" id="titlebar-minimize" onclick={minimize_window}>
            <img
                src="public/minimize.svg"
                alt="minimize"
                style="width: 20px; height: 20px;"/>
        </div>
        <div class="titlebar-button" id="titlebar-close" onclick={close_window}>
            <img
                src="public/close.svg"
                alt="close"
                style="width: 20px; height: 20px;"/>
        </div>
    </div>
</div>

    }
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(
            name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });

                || {}
            },
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <>
            <TitleBar />
            <main class="container">
                <h1>{"Welcome to Tauri + Yew"}</h1>

                <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                </div>
                <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                <form class="row" onsubmit={greet}>
                    <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                    <button type="submit">{"Greet"}</button>
                </form>
                <p>{ &*greet_msg }</p>
            </main>
        </>
    }
}
