use rand::Rng;

// constants
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// utility functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_f64_in_range(range: std::ops::Range<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(range)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
