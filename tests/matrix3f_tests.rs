#[cfg(test)]
mod tests {
    use ember_math::core::Matrix3f;
    use ember_math::core::Vector3f;

    static EPS:f32 = 1e-5;

    pub fn almost_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() < eps
    }

    pub fn matrix_seq() -> Matrix3f {
        let a = Matrix3f::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        );
        a
    }

    #[test]
    pub fn test_create_matrix3f(){
        let a = Matrix3f::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        );
        for i in 0..8 {
            assert_eq!(a.data[i], i as f32 + 1.0);
        }
    }

    #[test]
    pub fn test_matrix3f_one(){
        let a = Matrix3f::one();
        for i in 0..8 {
            assert_eq!(a.data[i], 1.0);
        }
    }

    #[test]
    pub fn test_matrix3f_zero(){
        let a = Matrix3f::zero();
        for i in 0..8 {
            assert_eq!(a.data[i], 0.0);
        }
    }

    #[test]
    pub fn test_matrix3_identity(){
        let a = Matrix3f::identity();
        assert_eq!(a.data[0], 1.0);
        assert_eq!(a.data[1], 0.0);
        assert_eq!(a.data[2], 0.0);

        assert_eq!(a.data[3], 0.0);
        assert_eq!(a.data[4], 1.0);
        assert_eq!(a.data[5], 0.0);

        assert_eq!(a.data[6], 0.0);
        assert_eq!(a.data[7], 0.0);
        assert_eq!(a.data[8], 1.0);
    }

    #[test]
    pub fn test_matrix3f_scale(){
        let a = matrix_seq();
        let b = a.scale(2.0);
        for i in 0..8 {
            assert_eq!(b.data[i], (i as f32 + 1.0)*2.0);
        }
    }

    #[test]
    pub fn test_matrix3f_transpose(){
        let mut a = matrix_seq();
        a = a.transpose();
        assert_eq!(a.data[0], 1.0);
        assert_eq!(a.data[1], 4.0);
        assert_eq!(a.data[2], 7.0);

        assert_eq!(a.data[3], 2.0);
        assert_eq!(a.data[4], 5.0);
        assert_eq!(a.data[5], 8.0);

        assert_eq!(a.data[6], 3.0);
        assert_eq!(a.data[7], 6.0);
        assert_eq!(a.data[8], 9.0);
    }

    #[test]
    pub fn test_determinant(){
        let a = matrix_seq();
        assert_eq!(a.determinant(), 0.0);

        let b = Matrix3f::new(
            1.0, 4.0, 3.0,
            -4.0, 0.0, 2.0,
            -7.0, -8.0, -9.0
        );
        assert_eq!(b.determinant(), -88.0);
    }

    #[test]
    pub fn test_transform(){
        let m1 = Matrix3f::identity();
        let v1 = Vector3f::new(1.0, 2.0, 3.0);
        let mv1 = m1.transform(v1);
        assert_eq!(mv1.x, 1.0);
        assert_eq!(mv1.y, 2.0);
        assert_eq!(mv1.z, 3.0);

        let m2 = Matrix3f::new(
            -1.0, 2.0, 4.5,
            1.43, 1.0, 93.2,
            0.0, 1.1, 0.5
        );
        let v2 = Vector3f::new(-0.2, 1.04, 12.2);
        let mv2 = m2.transform(v2);

        assert!(almost_eq(mv2.x, 57.18, EPS));
        assert!(almost_eq(mv2.y, 1137.793999, EPS));
        assert!(almost_eq(mv2.z, 7.244, EPS));
    }

    #[test]
    pub fn test_cofactor(){
        let m1 = matrix_seq();
        let m1c = m1.cofactor();
        assert_eq!(m1c.data[0], -3.0);
        assert_eq!(m1c.data[1], 6.0);
        assert_eq!(m1c.data[2], -3.0);

        assert_eq!(m1c.data[3], 6.0);
        assert_eq!(m1c.data[4], -12.0);
        assert_eq!(m1c.data[5], 6.0);

        assert_eq!(m1c.data[6], -3.0);
        assert_eq!(m1c.data[7], 6.0);
        assert_eq!(m1c.data[8], -3.0);
    }

    #[test]
    pub fn test_adjugate(){
        let m1 = matrix_seq();
        let adjugate = m1.adjugate();

        let result = Matrix3f::new(
            -3.0, 6.0, -3.0,
            6.0, -12.0, 6.0,
            -3.0, 6.0, -3.0
        );

        assert_eq!(adjugate, result);
    }

    #[test]
    pub fn test_inverse(){
        let m1 = Matrix3f::new(
            0.0, -3.0, -2.0,
            1.0, -4.0, -2.0,
            -3.0, 4.0, 1.0
        );
        let inv = m1.inverse();

        let r = Matrix3f::new(
            4.0, -5.0, -2.0,
            5.0, -6.0, -2.0,
            -8.0, 9.0, 3.0
        );
        assert_eq!(inv, r);
    }

    #[test]
    pub fn test_multiplication(){
        let m1 = Matrix3f::new(
            1.0, -2.0, 3.51,
            4.0, 5.0, 6.0,
            2.0, 3.0, 5.0
        );
        let m2 = Matrix3f::new(
            3.0, 2.0, 4.0,
            3.0, 3.0, 9.0, 
            4.0, 4.0, 2.0
        );
        let m12 = m1 * m2;

        let r12 = Matrix3f::new(
            11.04, 10.04, -6.98,
            51.0, 47.0, 73.0,
            35.0, 33.0, 45.0
        );
        assert_eq!(m12, r12);
    }

    #[test]
    pub fn test_add(){
        let m1 = Matrix3f::new(
            1.1, 2.2, -1.0,
            0.0, 1.0, 0.0,
            2.0, 1.0, -1.0
        );
        let m2 = Matrix3f::new(
            -0.01, 1.0, -0.3,
            1.0, 1.0, 1.0,
            1.2, 3.3, 43.0
        );
        let r = Matrix3f::new(
            1.09, 3.2, -1.3,
            1.0, 2.0, 1.0,
            3.2, 4.3, 42.0
        );
        assert_eq!(m1 + m2, r);
    }

    #[test]
    pub fn test_sub(){
        assert!(true);
    }

    #[test]
    pub fn test_equality() {
        let m1 = matrix_seq();
        let m2 = Matrix3f::new(
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0
        );
        assert_eq!(m1, m2);
    }

}