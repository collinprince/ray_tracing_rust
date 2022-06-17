use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;

use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Sync + Send>,
}

// constructor and setter functions
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material + Sync + Send>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
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
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: Arc::clone(&self.material),
        };
        let outward_normal: Vec3 = (hr.p - self.center) / self.radius;
        hr.set_face_normal(&r, outward_normal);
        Some(hr)
    }
}
