use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_f64;
use crate::vec3::{
    dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color, Vec3,
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

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // use schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Output> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64 = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction: Vec3 = unit_vector(r_in.direction());
        let cos_theta: f64 = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let schlick_approx: bool = self.reflectance(cos_theta, refraction_ratio) > random_f64();
        let direction: Vec3 = if cannot_refract || schlick_approx {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };
        let scattered: Ray = Ray::new(rec.p, direction);
        Some(Output {
            attenuation,
            scattered,
        })
    }
}
