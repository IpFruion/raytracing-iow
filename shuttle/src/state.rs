use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use image::RgbImage;
use uuid::Uuid;

use crate::{config::AppConfig, endpoints::status::ImageStatus};

pub type AppData = Arc<AppState>;
pub type CompletedImageGen = Result<RgbImage, anyhow::Error>;

pub struct AppState {
    pub img_gen: Mutex<HashMap<Uuid, ImageStatus<CompletedImageGen>>>,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(config: AppConfig) -> Arc<Self> {
        Arc::new(Self {
            img_gen: Default::default(),
            config,
        })
    }
}
