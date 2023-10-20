use std::{collections::hash_map::Entry, time::Duration};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer};
use tokio::task::spawn_blocking;
use tracing::{error, warn};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    state::AppData,
    utils::{anyhow_error_http_response, someting_went_wrong},
};

/// Get the status of image generation
#[utoipa::path(
    get,
    path = "/{id}",
    responses(
        (status = OK, description = "Image Status", body = ImageStatusResponse),
        (status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
)]
pub async fn image_status(
    State(state): State<AppData>,
    Path(id): Path<Uuid>,
) -> Result<ImageStatus<Json<CompletedImageResponse>>, (StatusCode, String)> {
    spawn_blocking(move || {
        let mut img_gen = state.img_gen.lock().map_err(|_e| {
            error!(message = "Unable to lock status");
            someting_went_wrong()
        })?;
        match img_gen.entry(id) {
            Entry::Occupied(o) => {
                let status = o
                    .get()
                    .as_ref()
                    .map_completed(|c| {
                        c.as_ref()
                            .map(|_| {
                                Json(CompletedImageResponse {
                                    download_url: format!(
                                        "{}/{}/download",
                                        state.config.root_url(),
                                        id
                                    ),
                                })
                            })
                            .map_err(anyhow_error_http_response)
                    })
                    .transpose_complete();

                if status.is_err() {
                    o.remove_entry();
                }
                status
            }
            Entry::Vacant(_) => {
                warn!(message = "Status Not found");
                Err((StatusCode::NOT_FOUND, "image id not found".to_string()))
            }
        }
    })
    .await
    .map_err(|e| {
        error!(message = "Unable to join status grab", err = ?e);
        someting_went_wrong()
    })?
}

#[derive(Serialize, ToSchema)]
pub struct CompletedImageResponse {
    // #[schema(example = "https://raytracing-iow.shuttle.rs/:id/download")]
    download_url: String,
}

#[derive(Clone, Serialize, ToSchema)]
pub struct Rendering {
    pub cur_pixel: u32,
    pub max_pixels: u32,
    pub percent: String,
    pub start: DateTime<Utc>,
    #[serde(serialize_with = "human_readable")]
    pub elapsed: Duration,
    #[serde(serialize_with = "human_readable")]
    pub eta: Duration,
}

impl Rendering {
    pub fn new(max_pixels: u32) -> Self {
        Self {
            cur_pixel: 0,
            max_pixels,
            percent: "0.00%".to_string(),
            start: Utc::now(),
            eta: Duration::ZERO,
            elapsed: Duration::ZERO,
        }
    }
}

fn human_readable<S: Serializer>(dur: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    let seconds = dur.as_secs() % 60;
    let minutes = (dur.as_secs() / 60) % 60;
    let hours = (dur.as_secs() / 60) / 60;
    let val = format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);
    serializer.serialize_str(&val)
}

#[derive(Clone, Serialize)]
pub enum ImageStatus<C> {
    Queued,
    Rendering(Rendering),
    Completed(C),
}

impl<C> ImageStatus<C> {
    pub fn as_ref(&self) -> ImageStatus<&C> {
        match *self {
            ImageStatus::Queued => ImageStatus::Queued,
            ImageStatus::Rendering(ref u) => ImageStatus::Rendering(u.clone()),
            ImageStatus::Completed(ref c) => ImageStatus::Completed(c),
        }
    }

    pub fn map_completed<O, F: FnOnce(C) -> O>(self, op: F) -> ImageStatus<O> {
        match self {
            ImageStatus::Queued => ImageStatus::Queued,
            ImageStatus::Rendering(u) => ImageStatus::Rendering(u),
            ImageStatus::Completed(c) => ImageStatus::Completed(op(c)),
        }
    }
}

impl<T, E> ImageStatus<Result<T, E>> {
    pub fn transpose_complete(self) -> Result<ImageStatus<T>, E> {
        match self {
            ImageStatus::Queued => Ok(ImageStatus::Queued),
            ImageStatus::Rendering(u) => Ok(ImageStatus::Rendering(u)),
            ImageStatus::Completed(c) => c.map(|t| ImageStatus::Completed(t)),
        }
    }
}

impl<C: IntoResponse> IntoResponse for ImageStatus<C> {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Queued => (StatusCode::ACCEPTED, Json(ImageStatus::<()>::Queued)).into_response(),
            Self::Rendering(u) => {
                (StatusCode::ACCEPTED, Json(ImageStatus::<()>::Rendering(u))).into_response()
            }
            Self::Completed(c) => c.into_response(),
        }
    }
}
