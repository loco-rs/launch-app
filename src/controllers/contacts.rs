#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::contacts::{ActiveModel, Model},
    views,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub email: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.email = Set(self.email.clone());
    }
}

#[debug_handler]
pub async fn new(
    ViewEngine(v): ViewEngine<TeraView>,
    State(_ctx): State<AppContext>,
) -> Result<Response> {
    views::contacts::create(&v)
}

#[debug_handler]
pub async fn add(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let is_exists = Model::exists(&ctx.db, &params.email).await?;

    if is_exists {
        tracing::debug!(email = params.email, "email already exists");
    } else {
        let mut item = ActiveModel {
            ..Default::default()
        };
        params.update(&mut item);
        item.insert(&ctx.db).await?;
    }
    views::contacts::show(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("contacts/")
        .add("/", post(add))
        .add("new", get(new))
}
