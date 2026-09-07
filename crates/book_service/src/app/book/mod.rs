use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::AppState;

mod handler;
mod payload;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list).post(handler::create))
        .route(
            "/{id}",
            get(handler::read)
                .put(handler::update)
                .delete(handler::delete),
        )
}

#[derive(OpenApi)]
#[openapi(
    paths(handler::list, handler::create, handler::read, handler::update, handler::delete),
    components(schemas(payload::BookRequest))
)]
pub struct BookApi;
