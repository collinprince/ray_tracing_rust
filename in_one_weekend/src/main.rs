mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use rtweekend::INFINITY;
use sphere::Sphere;
use vec3::{unit_vector, Color, Point3, Vec3};

use std::io::{self, Write};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec: hittable::HitRecord = hittable::HitRecord::new();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new().set_values(1.0, 1.0, 1.0));
    }
    let unit_dir: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new().set_values(1.0, 1.0, 1.0) + t * Color::new().set_values(0.5, 0.7, 1.0)
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // camera
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // world
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(
        Sphere::new().set_center_radius(Point3::new().set_values(0.0, 0.0, -1.0), 0.5),
    ));
    world.add(Box::new(Sphere::new().set_center_radius(
        Point3::new().set_values(0.0, -100.5, -1.0),
        100.0,
    )));

    let origin = Point3::new();
    let horizontal = Vec3::new().set_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new().set_values(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new().set_values(0.0, 0.0, focal_length);

    // render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u: f64 = (i as f64) / ((image_width - 1) as f64);
            let v: f64 = (j as f64) / ((image_height - 1) as f64);
            let r: Ray = Ray::new().set_fields(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color: Color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("\nDone.");
}
