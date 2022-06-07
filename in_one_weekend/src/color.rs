use crate::vec3;

pub fn write_color(pixel_color: vec3::Color) {
    println!(
        "{} {} {}",
        (255.999f64 * pixel_color.x()) as i32,
        (255.999f64 * pixel_color.y()) as i32,
        (255.999f64 * pixel_color.z()) as i32,
    )
}
