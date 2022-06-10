#[cfg(test)]
mod tests {
    use in_one_weekend::ray::*;
    use in_one_weekend::vec3::{Point3, Vec3};

    #[test]
    fn constructor_test() {
        let p: Point3 = Point3::new(1.0, 0.0, 0.0);
        let d: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(p, d);

        assert_eq!(r.origin(), p);
        assert_eq!(r.direction(), d);
    }

    #[test]
    fn at_test() {
        let p: Point3 = Point3::new(0.0, 0.0, 0.0);
        let d: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(p, d);

        let later_point: Point3 = Point3::new(3.0, 6.0, 9.0);

        assert_eq!(r.at(3.0), later_point);
    }
}
