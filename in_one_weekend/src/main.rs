mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::{random_f64, INFINITY};
use sphere::Sphere;
use vec3::{random_unit_vector, unit_vector, Color, Point3, Vec3};

use std::io::{self, Write};

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new().set_values(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let target: Point3 = rec.p + rec.normal + random_unit_vector();
        let reflected_ray: Ray = Ray::new().set_fields(rec.p, target - rec.p);
        return 0.5 * ray_color(&reflected_ray, world, depth - 1);
    }
    let unit_dir: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new().set_values(1.0, 1.0, 1.0) + t * Color::new().set_values(0.5, 0.7, 1.0)
}

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // world
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(
        Sphere::new().set_center_radius(Point3::new().set_values(0.0, 0.0, -1.0), 0.5),
    ));
    world.add(Box::new(Sphere::new().set_center_radius(
        Point3::new().set_values(0.0, -100.5, -1.0),
        100.0,
    )));

    // camera
    let cam: Camera = Camera::new();

    // render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Color = Color::new();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f64 = ((i as f64) + random_f64()) / ((IMAGE_WIDTH - 1) as f64);
                let v: f64 = ((j as f64) + random_f64()) / ((IMAGE_HEIGHT - 1) as f64);
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.");
}
