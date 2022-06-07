#[cfg(test)]
mod tests {
    use in_one_weekend::vec3::{cross, dot, unit_vector, Color, Point3, Vec3};
    #[test]
    fn create_default_vec3() {
        let v: Vec3 = Vec3::new();
        let zeroed_v: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };
        assert_eq!(v, zeroed_v);
    }

    #[test]
    fn create_vec3_with_params() {
        let v: Vec3 = Vec3::new().set_values(1.0, 2.0, 3.0);
        let corresponding_v: Vec3 = Vec3 { e: [1.0, 2.0, 3.0] };
        assert_eq!(v, corresponding_v);
    }

    #[test]
    fn check_x() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.x(), 1.0);
    }

    #[test]
    fn check_y() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.y(), 2.0);
    }

    #[test]
    fn check_z() {
        let v = Vec3 { e: [1.0, 2.0, 3.0] };
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn check_neg() {
        let mut v = Vec3::new().set_values(1.0, 2.0, 3.0);
        let v_neg = Vec3::new().set_values(-1.0, -2.0, -3.0);
        v = -v;
        assert_eq!(v, v_neg);
    }

    #[test]
    fn check_indexing_in_range() {
        let v = Vec3::new().set_values(1.0, 2.0, 3.0);
        assert_eq!(v[1], 2.0);
    }

    #[test]
    #[should_panic]
    fn check_indexing_out_of_range() {
        let v = Vec3::new().set_values(1.0, 2.0, 3.0);
        let _ = v[3];
    }

    #[test]
    fn check_mutable_indexing_in_range() {
        let mut v = Vec3::new().set_values(1.0, 2.0, 3.0);
        v[1] = 4.0;
        assert_eq!(v[1], 4.0);
    }

    #[test]
    #[should_panic]
    fn check_mutable_indexing_out_of_range() {
        let mut v = Vec3::new().set_values(1.0, 2.0, 3.0);
        v[3] = 10.0;
    }

    #[test]
    fn check_mul_assign() {
        let mut v = Vec3::new().set_values(1.0, 2.0, 3.0);
        let v_doubled = Vec3::new().set_values(2.0, 4.0, 6.0);
        v *= 2.0;
        assert_eq!(v, v_doubled);
    }

    #[test]
    fn check_div_assign() {
        let mut v = Vec3::new().set_values(2.0, 4.0, 6.0);
        let v_halved = Vec3::new().set_values(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, v_halved);
    }
    #[test]
    fn check_length() {
        let v = Vec3::new().set_values(3.0, 4.0, 5.0);
        let square: f64 = (3.0f64).powf(2.0) + (4.0f64).powf(2.0) + (5.0f64).powf(2.0);
        assert_eq!(v.length(), square.sqrt());
    }

    #[test]
    fn check_length_squared() {
        let v = Vec3::new().set_values(3.0, 4.0, 5.0);
        let square: f64 = (3.0f64).powf(2.0) + (4.0f64).powf(2.0) + (5.0f64).powf(2.0);
        assert_eq!(v.length_squared(), square);
    }

    #[test]
    fn check_point3_alias() {
        let p: Point3 = Point3::new().set_values(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
    }

    #[test]
    fn check_color_alias() {
        let c: Color = Color::new().set_values(1.0, 2.0, 3.0);
        assert_eq!(c.x(), 1.0);
        assert_eq!(c.y(), 2.0);
        assert_eq!(c.z(), 3.0);
    }

    #[test]
    fn format_test() {
        let v: Vec3 = Vec3::new().set_values(1.1, 2.2, 3.3);
        assert_eq!(format!("{v}"), "1.1 2.2 3.3");
    }

    #[test]
    fn add_test() {
        let u: Vec3 = Vec3::new().set_values(1.0, 1.0, 1.0);
        let v: Vec3 = Vec3::new().set_values(3.0, 2.0, 1.0);

        let result: Vec3 = Vec3::new().set_values(4.0, 3.0, 2.0);
        assert_eq!(u + v, result);
    }

    #[test]
    fn sub_test() {
        let u: Vec3 = Vec3::new().set_values(1.0, 1.0, 1.0);
        let v: Vec3 = Vec3::new().set_values(3.0, 2.0, 1.0);

        let result: Vec3 = Vec3::new().set_values(-2.0, -1.0, 0.0);
        assert_eq!(u - v, result);
    }

    #[test]
    fn mul_test() {
        let u: Vec3 = Vec3::new().set_values(2.0, 3.0, 4.0);
        let v: Vec3 = Vec3::new().set_values(2.0, 3.0, 4.0);

        let result: Vec3 = Vec3::new().set_values(4.0, 9.0, 16.0);
        assert_eq!(u * v, result);
    }

    #[test]
    fn mul_lhs_double_test() {
        let v: Vec3 = Vec3::new().set_values(2.0, 3.0, 4.0);
        let result: Vec3 = Vec3::new().set_values(6.0, 9.0, 12.0);

        assert_eq!(3.0 * v, result);
    }

    #[test]
    fn mul_rhs_double_test() {
        let v: Vec3 = Vec3::new().set_values(2.0, 3.0, 4.0);
        let result: Vec3 = Vec3::new().set_values(6.0, 9.0, 12.0);

        assert_eq!(v * 3.0, result);
    }

    #[test]
    fn div_test() {
        let v: Vec3 = Vec3::new().set_values(2.0, 3.0, 4.0);
        let result: Vec3 = Vec3::new().set_values(1.0, 1.5, 2.0);

        assert_eq!(v / 2.0, result);
    }

    #[test]
    fn dot_test() {
        let u = Vec3::new().set_values(1.0, 3.0, 5.0);
        let v = Vec3::new().set_values(2.0, 4.0, 6.0);

        assert_eq!(dot(&u, &v), 44.0);
    }

    #[test]
    fn cross_test() {
        let u = Vec3::new().set_values(1.0, 3.0, 5.0);
        let v = Vec3::new().set_values(2.0, 4.0, 6.0);

        let result = Vec3::new().set_values(-2.0, 4.0, -2.0);

        assert_eq!(cross(&u, &v), result);
    }

    #[test]
    fn unit_vector_test() {
        let v = Vec3::new().set_values(1.0, 2.0, 3.0);

        let v_length = (1.0f64.powi(2) + 2.0f64.powi(2) + 3.0f64.powi(2)).sqrt();
        let result = Vec3::new().set_values(1.0 / v_length, 2.0 / v_length, 3.0 / v_length);

        let unit_v = unit_vector(v);
        assert_eq!(unit_v, result);
        assert_eq!(unit_v.length(), 1.0);
    }
}
