use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use image::RgbImage;
use tokio::task::spawn_blocking;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::{
    state::AppData,
    utils::{anyhow_error_http_response, image_into_response},
};

use super::status::ImageStatus;

/// Downloads the processed image
#[utoipa::path(
    get,
    path = "/{id}/download",
    responses(
        (status = OK, description = "Image", body = String, content_type = "image/png"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
)]
pub async fn download_image(
    State(state): State<AppData>,
    Path(id): Path<Uuid>,
) -> Result<ImageResponse, DownloadError> {
    spawn_blocking(move || {
        let mut img_gen = state.img_gen.lock().map_err(|_e| {
            error!(message = "Unable to lock status");
            DownloadError::something_went_wrong()
        })?;
        let status = img_gen.remove(&id).ok_or_else(|| {
            warn!(message = "Image Id not found", id = %id);
            DownloadError::Error((StatusCode::NOT_FOUND, "image id not found".to_string()))
        })?;
        match status {
            ImageStatus::Completed(c) => c
                .map(|c| {
                    info!(message = "Finished Image", id = %id);
                    ImageResponse(c)
                })
                .map_err(|e| DownloadError::Error(anyhow_error_http_response(&e))),
            s => {
                let redirect_uri = format!("/{}", id);
                img_gen.insert(id, s);
                Err(DownloadError::Redirect(Redirect::to(&redirect_uri)))
            }
        }
    })
    .await
    .map_err(|e| {
        error!(message = "Unable to join status grab", err = ?e);
        DownloadError::something_went_wrong()
    })?
}

pub struct ImageResponse(RgbImage);

impl IntoResponse for ImageResponse {
    fn into_response(self) -> axum::response::Response {
        image_into_response(self.0).unwrap_or_else(|e| {
            error!(message = "Error occured during image writing", err = ?e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Error downloading image").into_response()
        })
    }
}

pub enum DownloadError {
    Error((StatusCode, String)),
    Redirect(Redirect),
}

impl DownloadError {
    #[inline]
    pub fn something_went_wrong() -> Self {
        DownloadError::Error((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        ))
    }
}

impl IntoResponse for DownloadError {
    fn into_response(self) -> axum::response::Response {
        match self {
            DownloadError::Error(e) => e.into_response(),
            DownloadError::Redirect(r) => r.into_response(),
        }
    }
}
