mod config;
mod endpoints;
mod models;
mod openapi;
mod render;
mod state;
mod utils;

use axum::{
    response::Redirect,
    routing::{get, post},
    Router,
};
use config::AppConfig;
use endpoints::{download::download_image, gen::gen_image, status::image_status};
use envconfig::Envconfig;
use openapi::ApiDoc;
use state::AppState;
use tower_http::services::ServeDir;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let config = AppConfig::init_from_env().map_err(anyhow::Error::from)?;
    let state = AppState::new(config.clone());

    info!(message = "Starting Raytracing In One Weekend", config = ?config);
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest_service("/ui", ServeDir::new("shuttle/assets"))
        .route("/", get(|| async { Redirect::to("/ui") }))
        .route("/", post(gen_image))
        .route("/:id", get(image_status))
        .route("/:id/download", get(download_image))
        .with_state(state);

    Ok(router.into())
}
