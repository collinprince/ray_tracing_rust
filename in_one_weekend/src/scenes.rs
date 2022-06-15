use crate::camera::*;
use crate::hittable::Hittable;
use crate::hittable_list::*;
use crate::material::*;
use crate::ray::Ray;
use crate::rtweekend::*;
use crate::sphere::*;
use crate::vec3::*;

use std::rc::Rc;

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some(Output {
            attenuation,
            scattered,
        }) = rec.material.scatter(r, &rec)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }
    let unit_dir: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub struct ImageSettings {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

pub struct SceneSettings {
    pub world: HittableList,
    pub cam: Camera,
    pub image_settings: ImageSettings,
}

// final render scene from in one weekend edition
fn random_scene() -> SceneSettings {
    let mut world: HittableList = HittableList::new();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.05, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material),
    )));

    let p: Point3 = Point3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64();
            let center: Point3 = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - p).length() > 0.9 {
                let mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Color = Color::random_in_range(0.5..1.0);
                    let fuzz: f64 = random_f64_in_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    // camera
    let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let aspect_ratio: f64 = 3.0 / 2.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let image_width: i32 = 1200;

    let image_settings: ImageSettings = ImageSettings {
        aspect_ratio: aspect_ratio,
        image_width: image_width,
        image_height: ((image_width as f64) / aspect_ratio) as i32,
        samples_per_pixel: 500,
        max_depth: 50,
    };

    SceneSettings {
        world,
        cam,
        image_settings,
    }
}

// defocus blur scene
pub fn defocus_blur_scene() -> SceneSettings {
    let mut world: HittableList = HittableList::new();

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let material_right: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        Rc::clone(&material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&material_right),
    )));

    // image settings
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth: i32 = 50;

    let image_settings: ImageSettings = ImageSettings {
        aspect_ratio,
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
    };

    // camera
    let lookfrom: Point3 = Point3::new(3.0, 3.0, 2.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = (lookfrom - lookat).length();
    let vfov: f64 = 20.0;
    let aperture = 2.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    SceneSettings {
        world,
        cam,
        image_settings,
    }
}
