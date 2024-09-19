#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn about(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(&v, "pages/about.html", serde_json::json!({}))
}

#[debug_handler]
pub async fn terms(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    format::render().view(&v, "pages/terms.html", serde_json::json!({}))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("pages/")
        .add("about", get(about))
        .add("terms", get(terms))
}
