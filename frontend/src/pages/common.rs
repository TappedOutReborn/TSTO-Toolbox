use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/crc32")]
    CRC32,
    #[at("/patcher")]
    Patcher,
    #[at("/portrait")]
    Portrait,
    #[not_found]
    #[at("/404")]
    NotFound,
}
