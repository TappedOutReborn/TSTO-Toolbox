use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" { 
    #[wasm_bindgen(js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, PartialEq)]
struct Tile {
    id: String,
    image_path: String,
}

#[derive(Properties, PartialEq)]
struct TileListProps {
    tiles: Vec<Tile>,
    on_click: Callback<Tile>,
}

#[function_component(TileList)]
fn tile_list(TileListProps { tiles, on_click }: &TileListProps) -> Html {
    tiles.iter().map(|tile| { 
        let on_tile_select = {
            let on_click = on_click.clone();
            let tile = tile.clone();
            Callback::from(move |_| {
                on_click.emit(tile.clone())
            })
        };

        html! {
            <div class="tile" onclick={on_tile_select}><div class="inner-circle"><img src={tile.image_path.clone()} /></div></div>
        }
    }).collect()
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
    let tiles = vec![
        Tile {
            id: "yearbook".to_string(),
            image_path: "public/tiles/yearbook.png".to_string(),
        },
        Tile {
            id: "shattered_dreams".to_string(),
            image_path: "public/tiles/shattered_dreams.png".to_string(),
        },
        Tile {
            id: "currency_presents".to_string(),
            image_path: "public/tiles/currency_presents.png".to_string(),
        },
        Tile {
            id: "yearbook".to_string(),
            image_path: "public/tiles/yearbook.png".to_string(),
        },
        Tile {
            id: "shattered_dreams".to_string(),
            image_path: "public/tiles/shattered_dreams.png".to_string(),
        },
        Tile {
            id: "currency_presents".to_string(),
            image_path: "public/tiles/currency_presents.png".to_string(),
        },
        Tile {
            id: "yearbook".to_string(),
            image_path: "public/tiles/yearbook.png".to_string(),
        },
        Tile {
            id: "shattered_dreams".to_string(),
            image_path: "public/tiles/shattered_dreams.png".to_string(),
        },
        Tile {
            id: "currency_presents".to_string(),
            image_path: "public/tiles/currency_presents.png".to_string(),
        },
        Tile {
            id: "yearbook".to_string(),
            image_path: "public/tiles/yearbook.png".to_string(),
        },
        Tile {
            id: "shattered_dreams".to_string(),
            image_path: "public/tiles/shattered_dreams.png".to_string(),
        },
        Tile {
            id: "currency_presents".to_string(),
            image_path: "public/tiles/currency_presents.png".to_string(),
        },
    ];

    let on_tile_select = {
        Callback::from(move |tile: Tile| {
            web_sys::console::log_1(&format!("Tile clicked: {}", tile.id).into());
        })
    };
 
    html! {
        <>
            <TitleBar />
            <main class="container">
                <div class="grid">
                    <TileList tiles={tiles} on_click={on_tile_select} />
                </div>
            </main>
        </>
    }
}
