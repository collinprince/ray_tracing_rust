use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{
    dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color, Vec3,
};

pub struct Output {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Output>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Output> {
        let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(Output {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        let fuzz: f64 = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Output> {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered: Ray = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if dot(&scattered.direction(), &rec.normal) > 0.0 {
            let attenuation = self.albedo;
            Some(Output {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
