mod renderer;
use std::{env::args, io};

use rand::{rngs::SmallRng, Rng, SeedableRng};
use raytracing_iow::{
    color::{Color, WHITE},
    materials::{dielectric::Dielectric, lambertian::Lambertain, metal::Metal},
    render::{
        camera::{Camera, CameraConfig},
        screen::Screen,
        viewport::ViewportConfig,
    },
    shapes::sphere::Sphere,
    vec3::Vec3,
    world::{Object, World},
};

use crate::renderer::Renderer;

const ASPECT_RATIO: f64 = 16. / 9.;
const ORANGE: Color = Color::new(250. / 256., 121. / 256., 35. / 256.);
const PURPLE: Color = Color::new(71. / 256., 5. / 256., 158. / 256.);

fn main() -> io::Result<()> {
    let mut filename = args().nth(1).unwrap_or("rendering".to_owned());
    filename.push_str(".ppm");

    let screen = Screen::new_aspect_ratio(1200, ASPECT_RATIO);
    let camera_config = CameraConfig {
        samples_per_pixel: 500,
        max_depth: 50,
        pos: (13., 2., 3.).into(),
        look_at: (0., 0., 0.).into(),
        up: (0., 1., 0.).into(),
        defocus_angle: 0.6,
        focus_dist: 10.0,
    };
    let viewport_config = ViewportConfig::Fov { vertical_fov: 20.0 };

    let camera = Camera::new(camera_config, viewport_config);

    let mut world = vec![
        Object::new(
            Sphere::new((0., -1000., -1.), 1000.),
            Lambertain::new(ORANGE),
        ), // land
        Object::new(Sphere::new((0., 1., 0.), 1.0), Dielectric::new(1.5)), // ball
        Object::new(Sphere::new((-4., 1., 0.), 1.0), Lambertain::new(PURPLE)), // ball
        Object::new(Sphere::new((4., 1., 0.), 1.0), Metal::new(WHITE, 0.)), // ball
    ];

    let mut rng = SmallRng::from_entropy();
    for i in -11..11 {
        for j in -11..11 {
            let x = i as f64 + 0.9 * rng.gen::<f64>();
            let z = j as f64 + 0.9 * rng.gen::<f64>();
            let color = Color::from(rng.gen::<Vec3>());
            let from = Vec3::new(x, 0.2, z);
            let to = from + Vec3::new(0., rng.gen_range(0.0..0.5), 0.);

            let shape = Sphere::new_moving(from, to, 0.2);
            let obj = match rng.gen_range(0..3) {
                0 => Object::new(shape, Metal::new(color, 0.3)),
                2 => Object::new(shape, Dielectric::new(rng.gen_range(0.5..2.0))),
                _ => Object::new(shape, Lambertain::new(color)),
            };
            world.push(obj)
        }
    }
    println!("Setup World Starting Render");

    let mut renderer = Renderer::new(screen, &filename)?;
    renderer.render(&camera, &World::from(world))?;

    Ok(())
}
