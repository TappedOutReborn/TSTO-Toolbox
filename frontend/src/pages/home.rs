use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use yew::prelude::*;
use yew_router::{navigator, prelude::*};

use super::common::Route;

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

#[function_component(Home)]
pub fn home() -> Html {
    let tiles = vec![
        Tile {
            id: "crc32".to_string(),
            image_path: "public/tiles/yearbook.png".to_string(),
        },
        Tile {
            id: "patcher".to_string(),
            image_path: "public/tiles/medkit.png".to_string(),
        },
        Tile {
            id: "portrait".to_string(),
            image_path: "public/tiles/ico_popup_screenshotmode.png".to_string(),
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
        Tile {
            id: "shattered_dreams".to_string(),
            image_path: "public/tiles/shattered_dreams.png".to_string(),
        },
    ];

    let on_tile_select = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |tile: Tile| {
            web_sys::console::log_1(&format!("Tile clicked: {}", tile.id).into()); // Debug
            
            match tile.id.as_str() {
                "crc32" => navigator.replace(&Route::CRC32),
                "patcher" => navigator.replace(&Route::Patcher),
                "portrait" => navigator.replace(&Route::Portrait),
                _ => { /* Everything Else: Do Nothing */ },
            }
        })
    };

    html! {
        <div class="grid">
            <TileList tiles={tiles} on_click={on_tile_select} />
        </div>
    }
}

