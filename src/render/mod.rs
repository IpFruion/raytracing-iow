pub mod camera;
pub mod screen;
pub mod viewport;

use std::{
    fmt::Display,
    fs::File,
    io::{self, BufWriter, Seek, SeekFrom, Write},
    ops::Range,
    path::Path,
};

use indicatif::ProgressBar;
use rand::{
    distributions::Standard,
    rngs::{SmallRng, ThreadRng},
    Rng, SeedableRng,
};

use crate::{
    color::{Color, BLACK},
    ray::Ray,
    shapes::Hittable,
    vec3::Vec3,
    world::World,
};

use self::{
    camera::{Camera, Defocus},
    screen::Screen,
};

pub struct Renderer {
    screen: Screen,
    writer: BufWriter<File>,
}

impl Renderer {
    pub fn new<P: AsRef<Path>>(screen: Screen, output_file: P) -> io::Result<Self> {
        Ok(Renderer {
            screen,
            writer: BufWriter::new(File::create(output_file)?),
        })
    }

    pub fn render(&mut self, camera: &mut Camera, world: &World) -> io::Result<()> {
        let width = self.screen.width();
        let height = self.screen.height();

        let bar = ProgressBar::new(width * height);

        self.writer.seek(SeekFrom::Start(0))?;
        write!(self.writer, "P3\n{} {}\n255\n\n", width, height)?;

        let pixel_locator = PixelLocator::from_screen_and_camera(&self.screen, &camera);

        for y in 0..height {
            for x in 0..width {
                let pixel_center = pixel_locator.pixel_center(x, y);
                let pixel = camera.get_color(world, &pixel_locator, pixel_center);
                write!(self.writer, "{}\n", pixel)?;
                bar.inc(1);
            }
        }

        Ok(())
    }
}

pub struct PixelLocator {
    delta_u: Vec3,
    delta_v: Vec3,
    upper_left_loc: Vec3,
    defocus: Defocus,
}

impl PixelLocator {
    fn from_screen_and_camera(screen: &Screen, camera: &Camera) -> Self {
        let (viewport, defocus) = camera.viewport(screen);
        let delta_u = viewport.u / (screen.width() as f64);
        let delta_v = viewport.v / (screen.height() as f64);
        Self {
            delta_u,
            delta_v,
            upper_left_loc: viewport.upper_left + 0.5 * (delta_u + delta_v),
            defocus,
        }
    }

    pub fn adjust_pixel_loc(&self, pixel_loc: Vec3, dx: f64, dy: f64) -> Vec3 {
        pixel_loc + (dx * self.delta_u) + (dy * self.delta_v)
    }

    pub fn defocus_pixel(&self, pixel_loc: Vec3, dx: f64, dy: f64) -> Vec3 {
        pixel_loc + (dx * self.defocus.disk_u) + (dy * self.defocus.disk_v)
    }

    fn pixel_center(&self, x: u64, y: u64) -> Vec3 {
        self.upper_left_loc + ((x as f64) * self.delta_u) + ((y as f64) * self.delta_v)
    }
}