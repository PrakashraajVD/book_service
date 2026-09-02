use axum::{Router, routing::get};

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
