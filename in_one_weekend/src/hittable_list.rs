use crate::hittable::*;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

// constructors
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn initial_object(self, object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }
}

// modifiers
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut closest_so_far: f64 = t_max;
        let mut hit_anything: bool = false;

        for object in self.objects.iter() {
            if let Some(hr) = object.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                temp_rec = hr;
            }
        }
        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
    }
}
