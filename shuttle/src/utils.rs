use anyhow::anyhow;
use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use image::{ImageFormat, RgbImage};
use std::{
    io::{BufWriter, Cursor},
    sync::PoisonError,
};

#[inline]
pub fn map_poison_error<I>(e: PoisonError<I>) -> anyhow::Error {
    anyhow!("Poison Error: {}", e)
}

pub fn image_into_response(img: RgbImage) -> Result<axum::response::Response, anyhow::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "image/png".parse()?);

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    img.write_to(&mut buffer, ImageFormat::Png)?;

    let img = buffer.into_inner()?.into_inner();
    headers.insert(header::CONTENT_LENGTH, img.len().into());

    Ok((StatusCode::OK, headers, img).into_response())
}

#[inline]
pub fn someting_went_wrong() -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong".to_string(),
    )
}

#[inline]
pub fn anyhow_error_http_response(e: &anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("Error occured during processing: {}", e),
    )
}
