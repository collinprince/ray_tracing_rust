mod tests {
    use in_one_weekend::hittable::{HitRecord, Hittable};
    use in_one_weekend::ray::*;
    use in_one_weekend::sphere::*;
    use in_one_weekend::vec3::*;

    #[test]
    fn goes_through_sphere_test() {
        let origin: Point3 = Point3::new();
        let s: Sphere = Sphere::new().set_center_radius(origin, 1.0);

        let dir: Vec3 = Vec3::new().set_values(1.0, 0.0, 0.0);
        let r: Ray = Ray::new().set_fields(origin, dir);

        let hit: HitRecord = s.hit(&r, 0.0, f64::INFINITY).unwrap();
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.p, Point3::new().set_values(1.0, 0.0, 0.0));
        assert_eq!(hit.front_face, false);
        assert_eq!(hit.normal, -(hit.p - s.center));
    }

    #[test]
    fn does_not_hit_sphere_test() {
        let origin: Point3 = Point3::new();
        let s: Sphere = Sphere::new().set_center_radius(origin, 1.0);

        let r_origin: Point3 = Point3::new().set_values(1.0, 2.0, 2.0);
        let dir: Vec3 = Vec3::new().set_values(2.0, 2.0, 2.0);
        let r: Ray = Ray::new().set_fields(r_origin, dir);

        let hit: Option<HitRecord> = s.hit(&r, 0.0, f64::INFINITY);
        assert!(hit.is_none());
    }

    #[test]
    fn hits_at_one_point_test() {
        let origin: Point3 = Point3::new();
        let s: Sphere = Sphere::new().set_center_radius(origin, 1.0);

        let intersect_p: Point3 = Point3::new().set_values(0.0, 1.0, 0.0);
        let r_origin: Point3 = Point3::new().set_values(-1.0, 1.0, 0.0);
        let r_dir: Vec3 = intersect_p - r_origin;
        let r: Ray = Ray::new().set_fields(r_origin, r_dir);

        let hit: HitRecord = s.hit(&r, 0.0, f64::INFINITY).unwrap();
        let normal: Vec3 = -(intersect_p - origin);
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.p, intersect_p);
        assert_eq!(hit.front_face, false);
        assert_eq!(hit.normal, normal);
    }
}
