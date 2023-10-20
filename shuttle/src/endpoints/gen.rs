use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use raytracing_iow::render::camera::CameraConfig;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use tracing::{error, info};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    endpoints::status::ImageStatus,
    models::{Color, Material, Object, Shape, Sphere},
    render::render_img,
    state::AppData,
    utils::someting_went_wrong,
};

const MAX_PROCESSING: usize = 3;
const MAX_DIM: u32 = 3000;
const MAX_SAMPLES: u32 = 500;
const MAX_DEPTH: u32 = 50;

/// Starts the generation of an image
#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = OK, description = "Image Generation Started", body = GenImageResponse),
        (status = TOO_MANY_REQUESTS, description = "Too many image generation requests come back later"),
        (status = INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
)]
pub async fn gen_image(
    State(state): State<AppData>,
    Json(req): Json<GenImageRequest>,
) -> Result<Json<GenImageResponse>, (StatusCode, String)> {
    req.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let new_id = Uuid::new_v4();

    info!(message = "Starting New Image Request", id = %new_id);
    let queue_state = state.clone();
    spawn_blocking(move || queue_req(new_id, queue_state))
        .await
        .map_err(|e| {
            error!(message = "Failed to join queue image gen handler", err = ?e);
            someting_went_wrong()
        })??;

    info!(message = "Spawning Generating Image");
    let render_state = state.clone();
    spawn_blocking(move || {
        if let Err(e) = render_img(&render_state, &new_id, req) {
            error!(message = "Error when processing image", err = ?e);
            match render_state.img_gen.lock() {
                Ok(mut img_gen) => {
                    if let Some(status) = img_gen.get_mut(&new_id) {
                        *status = ImageStatus::Completed(Err(e))
                    }
                }
                Err(e) => {
                    error!(message = "Failed to get write lock for queueing image gen", err = ?e)
                }
            }
        }
    });

    let status_url = format!("{}/{}", state.config.root_url(), new_id);
    Ok(Json(GenImageResponse {
        id: new_id,
        status_url,
    }))
}

fn queue_req(id: Uuid, state: AppData) -> Result<(), (StatusCode, String)> {
    let mut img_gen = state.img_gen.lock().map_err(|e| {
        error!(message = "Failed to get write lock for queueing image gen", err = ?e);
        someting_went_wrong()
    })?;
    if img_gen.len() > MAX_PROCESSING {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Too many image gen requests come back later".to_string(),
        ));
    }
    img_gen.insert(id, ImageStatus::Queued);
    Ok(())
}

#[derive(ToSchema, Deserialize)]
pub struct GenImageRequest {
    #[schema(example = 300, maximum = 3000, minimum = 1)]
    pub width: u32,

    #[schema(example = 500, maximum = 3000, minimum = 1)]
    pub height: u32,

    #[schema(default = json!({
        "samples_per_pixel": 100,
        "max_depth": 50,
        "pos": { "x": 13., "y": 2., "z": 3.},
        "look_at": { "x": 0., "y": 0., "z": 0.},
        "up": { "x": 0., "y": 1., "z": 0.},
        "defocus_angle": 0.6,
        "focus_dist": 10.0,
    }))]
    #[serde(default = "default_camera_config")]
    pub camera_config: CameraConfig,

    #[schema(default = json!([
        {
            "material": {"Lambertain": { "color": { "r": 0.9765625, "g": 0.47265625, "b": 0.13671875 }}},
            "shape": {"Sphere": {"Stationary": {"center": { "x": 0., "y": -1000., "z": 0.}, "radius": 1000.0}}}
        },
        {
            "material": {"Dielectric": { "index_of_refraction": 1.5 }},
            "shape": {"Sphere": {"Stationary": {"center": { "x": 0., "y": 1., "z": 0.}, "radius": 1.}}}
        },
        {
            "material": {"Lambertain": { "color": { "r": 0.27734375, "g": 0.01953125, "b": 0.6171875 }}},
            "shape": {"Sphere": {"Stationary": {"center": { "x": -4., "y": 1., "z": 0.}, "radius": 1.}}}
        },
        {
            "material": {"Metal": { "color": { "r": 1.0, "g": 1.0, "b": 1.0 }, "fuzziness": 0.0 }},
            "shape": {"Sphere": {"Stationary": {"center": { "x": 4., "y": 1., "z": 0.}, "radius": 1.}}}
        }
    ]))]
    #[serde(default = "default_objects")]
    pub objects: Vec<Object>,
}

impl GenImageRequest {
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.width > MAX_DIM || self.width < 1 || self.height > MAX_DIM || self.height < 1 {
            return Err(anyhow!(
                "Width or Height: ({}, {}) can't be greater than {} pixels and less than 1",
                self.width,
                self.height,
                MAX_DIM,
            ));
        }
        if self.camera_config.samples_per_pixel > MAX_SAMPLES {
            return Err(anyhow!(
                "Too many Samples per Pixel: {} out of {}",
                self.camera_config.samples_per_pixel,
                MAX_SAMPLES,
            ));
        }
        if self.camera_config.max_depth > MAX_DEPTH {
            return Err(anyhow!(
                "Max Depth too big: {} out of {}",
                self.camera_config.samples_per_pixel,
                MAX_DEPTH
            ));
        }

        Ok(())
    }
}

#[derive(Serialize, ToSchema)]
pub struct GenImageResponse {
    id: Uuid,
    status_url: String,
}

fn default_camera_config() -> CameraConfig {
    CameraConfig {
        samples_per_pixel: 100,
        max_depth: 50,
        pos: (13., 2., 3.).into(),
        look_at: (0., 0., 0.).into(),
        up: (0., 1., 0.).into(),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    }
}

fn default_objects() -> Vec<Object> {
    vec![
        Object {
            shape: Shape::Sphere(Sphere::Stationary {
                center: (0., -1000., 0.).into(),
                radius: 1000.,
            }),
            material: Material::Lambertain {
                color: Color {
                    r: 0.9765625,
                    g: 0.47265625,
                    b: 0.13671875,
                },
            },
        },
        Object {
            shape: Shape::Sphere(Sphere::Stationary {
                center: (0., 1., 0.).into(),
                radius: 1.,
            }),
            material: Material::Metal {
                color: Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                },
                fuzziness: 0.,
            },
        },
        Object {
            shape: Shape::Sphere(Sphere::Stationary {
                center: (-4., 1., 0.).into(),
                radius: 1.,
            }),
            material: Material::Lambertain {
                color: Color {
                    r: 71. / 256.,  //0.27734375
                    g: 5. / 256.,   //0.01953125
                    b: 158. / 256., //0.6171875
                },
            },
        },
        Object {
            shape: Shape::Sphere(Sphere::Stationary {
                center: (4., 1., 0.).into(),
                radius: 1.,
            }),
            material: Material::Metal {
                color: Color {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                },
                fuzziness: 0.,
            },
        },
    ]
}
