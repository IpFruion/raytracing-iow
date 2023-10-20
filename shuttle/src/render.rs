use std::time::Duration;

use anyhow::anyhow;
use chrono::Utc;
use image::{ImageBuffer, Rgb, RgbImage};
use raytracing_iow::{
    render::{camera::Camera, screen::Screen, viewport::ViewportConfig, PixelLocator},
    world::{Object, World},
};
use uuid::Uuid;

use crate::{
    endpoints::{
        gen::GenImageRequest,
        status::{ImageStatus, Rendering},
    },
    state::AppData,
    utils::map_poison_error,
};

pub fn render_img(state: &AppData, id: &Uuid, req: GenImageRequest) -> Result<(), anyhow::Error> {
    let mut img: RgbImage = ImageBuffer::new(req.width, req.height);

    let progress = Progress::new(state, id);
    progress.start(req.width * req.height)?;

    let screen = Screen::new(req.width.into(), req.height.into());
    let viewport_config = ViewportConfig::Fov { vertical_fov: 20.0 };

    let camera = Camera::new(req.camera_config, viewport_config);

    let pixel_locator = PixelLocator::from_screen_and_camera(&screen, &camera);
    let world: World = req
        .objects
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Object>>()
        .into();

    for y in 0..req.height {
        for x in 0..req.width {
            let pixel_center = pixel_locator.pixel_center(x.into(), y.into());

            let pixel = camera
                .get_color(&world, &pixel_locator, pixel_center)
                .into_arr();

            img.put_pixel(
                x,
                y,
                Rgb([
                    (pixel[0] * 255.).trunc() as u8,
                    (pixel[1] * 255.).trunc() as u8,
                    (pixel[2] * 255.).trunc() as u8,
                ]),
            );
            progress.inc()?;
        }
    }

    progress.complete(img)?;

    Ok(())
}

struct Progress<'a> {
    state: &'a AppData,
    id: &'a Uuid,
}

impl<'a> Progress<'a> {
    pub fn new(state: &'a AppData, id: &'a Uuid) -> Self {
        Self { state, id }
    }

    pub fn start(&self, pixels: u32) -> Result<(), anyhow::Error> {
        let mut img_gen = self.state.img_gen.lock().map_err(map_poison_error)?;
        let state = img_gen
            .get_mut(self.id)
            .ok_or(anyhow!("Image Gen not Started"))?;

        *state = ImageStatus::Rendering(Rendering::new(pixels));

        Ok(())
    }

    pub fn inc(&self) -> Result<(), anyhow::Error> {
        let mut img_gen = self.state.img_gen.lock().map_err(map_poison_error)?;
        let state = img_gen
            .get_mut(self.id)
            .ok_or(anyhow!("Image Gen not Started"))?;

        match state {
            ImageStatus::Rendering(r) => {
                r.cur_pixel += 1;
                r.elapsed = (Utc::now() - r.start).to_std()?;
                let diff = r.max_pixels - r.cur_pixel;
                let pixels_per_second = r.elapsed.as_secs() as f64 / r.cur_pixel as f64;

                let eta_secs = diff as f64 * pixels_per_second;
                r.eta = Duration::from_secs(eta_secs as u64);

                r.percent = format!("{:.5}%", (r.cur_pixel as f64 / r.max_pixels as f64) * 100.);
            }
            _ => (),
        }

        Ok(())
    }

    pub fn complete(&self, img: RgbImage) -> Result<(), anyhow::Error> {
        let mut img_gen = self.state.img_gen.lock().map_err(map_poison_error)?;

        let state = img_gen
            .get_mut(self.id)
            .ok_or(anyhow!("Image Gen not Started"))?;

        *state = ImageStatus::Completed(Ok(img));
        Ok(())
    }
}
