use crate::ray::Ray;
use crate::vec3::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;

        let origin = Point3::new().set_values(0.0, 0.0, 0.0);
        let horizontal = Vec3::new().set_values(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new().set_values(0.0, viewport_height, 0.0);
        let focal_vec = Vec3::new().set_values(0.0, 0.0, focal_length);

        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_vec;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new().set_fields(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn lower_left_corner(&self) -> Point3 {
        self.lower_left_corner
    }

    pub fn horizontal(&self) -> Point3 {
        self.horizontal
    }

    pub fn vertical(&self) -> Point3 {
        self.vertical
    }
}
