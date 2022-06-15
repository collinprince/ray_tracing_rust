mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod scenes;
mod sphere;
mod vec3;

use color::write_color;
use hittable_list::HittableList;
use material::*;
use ray::Ray;
use rtweekend::{random_f64, random_f64_in_range};
use scenes::{ray_color, ImageSettings, SceneSettings};
use sphere::Sphere;
use vec3::{Color, Point3};

use std::io::{self, Write};
use std::rc::Rc;

fn main() {
    // get scene, cam, and settings
    let SceneSettings {
        world,
        cam,
        image_settings,
    } = scenes::defocus_blur_scene();
    let ImageSettings {
        aspect_ratio: _,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    } = image_settings;

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u: f64 = ((i as f64) + random_f64()) / ((image_width - 1) as f64);
                let v: f64 = ((j as f64) + random_f64()) / ((image_height - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}
