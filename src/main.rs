#![allow(unused)]
mod color;
mod material;
mod ray;
mod render;
mod shapes;
mod utils;
mod vec3;
mod world;

use std::{env::args, io, ops::Deref};

use color::{Color, ORANGE, PURPLE, SKY_BLUE};
use material::{dielectric::Dielectric, lambertian::Lambertain, metal::Metal};
use ray::Ray;
use render::{camera::Camera, screen::Screen, Renderer};
use shapes::sphere::Sphere;
use vec3::{Vec3, ONE};
use world::Object;

const WIDTH: u64 = 900;
const ASPECT_RATIO: f64 = 16. / 9.;

fn main() -> io::Result<()> {
    let mut filename = args().skip(1).next().unwrap_or("rendering".to_owned());
    filename.push_str(".ppm");

    let screen = Screen::new_aspect_ratio(WIDTH, ASPECT_RATIO);

    let viewport = screen.viewport(2.);
    let mut camera = Camera::new((0., 0., 0.).into(), 1., viewport, 20, 50);

    let world = vec![
        Object::new_boxed(
            Sphere::new((0., -100.5, -1.), 100.),
            Lambertain::new(ORANGE),
        ), // land
        Object::new_boxed(Sphere::new((0., 0., -1.), 0.5), Lambertain::new(PURPLE)), // ball
        Object::new_boxed(Sphere::new((-1., 0., -1.), 0.5), Metal::new(SKY_BLUE, 0.3)), // ball
        Object::new_boxed(Sphere::new((1., 0., -1.), 0.5), Dielectric::new(1.5)),    // ball
        Object::new_boxed(Sphere::new((1., 0., -1.), -0.4), Dielectric::new(1.5)),   // ball
    ];

    let mut renderer = Renderer::new(screen, &filename)?;
    renderer.render(&mut camera, &world)?;

    Ok(())
}
