#![allow(unused)]
mod material;
mod pixel;
mod ray;
mod render;
mod shapes;
mod utils;
mod vec3;

use std::{env::args, io, ops::Deref};

use material::{lambertian::Lambertain, metal::Metal};
use pixel::{Pixel, ORANGE, PURPLE, SKY_BLUE};
use ray::Ray;
use render::{camera::Camera, hit::Hittable, screen::Screen, Renderer};
use shapes::sphere::Sphere;
use vec3::{Vec3, ONE};

const WIDTH: u64 = 900;
const ASPECT_RATIO: f64 = 16. / 9.;

fn main() -> io::Result<()> {
    let mut filename = args().skip(1).next().unwrap_or("rendering".to_owned());
    filename.push_str(".ppm");

    let screen = Screen::new_aspect_ratio(WIDTH, ASPECT_RATIO);

    let viewport = screen.viewport(2.);
    let mut camera = Camera::new((0., 0., 0.).into(), 1., viewport, 10, 50);

    let world = vec![
        Sphere::new(Lambertain::new(PURPLE), (0.5, 0., -1.), 0.5).boxed(), // ball
        Sphere::new(Metal::new(SKY_BLUE), (-0.5, 0., -1.), 0.5).boxed(),   // land
        Sphere::new(Lambertain::new(ORANGE), (0., -100.5, -1.), 100.).boxed(), // land
    ];

    let mut renderer = Renderer::new(screen, &filename)?;
    renderer.render(&mut camera, &world)?;

    Ok(())
}
