use crate::rtweekend::clamp;
use crate::vec3::Color;

pub fn color_to_string(pixel_color: Color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // divide the color by the number of samples
    r = (r / (samples_per_pixel as f64)).sqrt();
    g = (g / (samples_per_pixel as f64)).sqrt();
    b = (b / (samples_per_pixel as f64)).sqrt();

    format!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u64,
        (256.0 * clamp(g, 0.0, 0.999)) as u64,
        (256.0 * clamp(b, 0.0, 0.999)) as u64,
    )
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    println!("{}", color_to_string(pixel_color, samples_per_pixel));
}
