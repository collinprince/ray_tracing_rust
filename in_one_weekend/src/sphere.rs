use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

// constructor and setter functions
impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Vec3::new(),
            radius: 0.0,
        }
    }

    pub fn set_center_radius(self, c: Vec3, r: f64) -> Sphere {
        Sphere {
            center: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f64 = r.direction().length_squared();
        let half_b: f64 = dot(&oc, &r.direction());
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / a;
        // let root_bounded = |root| -> bool { root < t_min || t_max < root };
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hr = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(),
            front_face: false,
        };
        let outward_normal: Vec3 = (hr.p - self.center) / self.radius;
        hr.set_face_normal(&r, outward_normal);
        Some(hr)
    }
}
