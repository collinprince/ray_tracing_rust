#[cfg(test)]
mod tests {
    use in_one_weekend::camera::*;
    use in_one_weekend::ray::Ray;
    use in_one_weekend::vec3::*;

    #[test]
    fn get_ray_test_one() {
        let cam: Camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
            0.0,
            1.0,
        );
        let r: Ray = cam.get_ray(0.0, 0.0);
        assert_eq!(r.origin(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction(), cam.lower_left_corner() - cam.origin());
    }

    #[test]
    fn get_ray_test_two() {
        let cam: Camera = Camera::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
            0.0,
            1.0,
        );
        let u: f64 = 0.5;
        let v: f64 = 0.9;
        let r: Ray = cam.get_ray(u, v);

        // assert_eq!(cam.vertical(), Vec3::new(0.0, 2.0, 0.0));
        // assert_eq!(cam.horizontal(), Vec3::new(2.0 * (16.0 / 9.0), 0.0, 0.0));
        assert_eq!(r.origin(), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(
            r.direction(),
            cam.lower_left_corner() + u * cam.horizontal() + v * cam.vertical() - cam.origin()
        );
    }
}
