#[cfg(test)]
mod tests {
    use in_one_weekend::ray::*;
    use in_one_weekend::vec3::{Point3, Vec3};

    #[test]
    fn blank_constructor_test() {
        let r = Ray::new();

        let correct_orig = Point3 { e: [0.0, 0.0, 0.0] };
        let correct_dir = Vec3 { e: [0.0, 0.0, 0.0] };
        assert_eq!(r.orig, correct_orig);
        assert_eq!(r.dir, correct_dir);
    }

    #[test]
    fn constructor_with_fields_test() {
        let p: Point3 = Point3::new().set_values(1.0, 0.0, 0.0);
        let d: Vec3 = Vec3::new().set_values(1.0, 2.0, 3.0);
        let r = Ray::new().set_fields(p, d);

        assert_eq!(r.origin(), p);
        assert_eq!(r.direction(), d);
    }

    #[test]
    fn at_test() {
        let p: Point3 = Point3::new().set_values(0.0, 0.0, 0.0);
        let d: Vec3 = Vec3::new().set_values(1.0, 2.0, 3.0);
        let r = Ray::new().set_fields(p, d);

        let later_point: Point3 = Point3::new().set_values(3.0, 6.0, 9.0);

        assert_eq!(r.at(3.0), later_point);
    }
}
