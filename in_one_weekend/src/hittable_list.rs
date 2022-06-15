use crate::hittable::*;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

// constructors
impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    #[allow(dead_code)]
    pub fn initial_object(self, object: Box<dyn Hittable + Sync + Send>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }
}

// modifiers
impl HittableList {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if let Some(hr) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hr.t;
                temp_rec = Some(hr);
            }
        }
        temp_rec
    }
}
