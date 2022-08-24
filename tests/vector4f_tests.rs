#[cfg(test)]
mod tests {
    use ember_math::core::Vector4f;

    static EPS:f32 = 0.0000001;

    #[test]
    pub fn test_create_vector4f(){
        let a = Vector4f::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 1.0);
        assert_eq!(a.z, 1.0);
        assert_eq!(a.w, 1.0);
    }

    #[test]
    pub fn test_vector4f_equality(){
        let a = Vector4f::new(-1.0, 1.0, 3.0, -1.2524);
        let b = Vector4f::new(-1.0, 1.0, 3.0, -1.2524);
        assert_eq!(a, b);
    }

    #[test]
    pub fn test_vector4f_magnitude(){
        let a = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let b = Vector4f::new(1.0, 1.0, 1.0, 1.0);
        assert!((b.magnitude() - 2.0).abs() < EPS);
        assert_eq!(1.0, a.magnitude());
    }

    #[test]
    pub fn test_vector4f_magnitude_squared(){
        let a = Vector4f::new(3.0, 4.0, 1.0, -1.0);
        let b = Vector4f::new(-1.0, -2.0, 2.0, 2.0);
        let c = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(27.0, a.magnitude_squared());
        assert_eq!(13.0, b.magnitude_squared());
        assert_eq!(0.0, c.magnitude_squared());
    }

    #[test]
    pub fn test_vector4f_one(){
        let a = Vector4f::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(a, Vector4f::one());
    }

    #[test]
    pub fn test_vector4f_zero(){
        let a = Vector4f::new(0.0, 0.0, 0.0, 0.0);
        assert_eq!(Vector4f::zero(), a);
    }

    #[test]
    pub fn test_vector4f_dot(){
        let a = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let b = Vector4f::new(0.0, 1.0, 0.0, 0.0);
        let ab = a.dot(b);

        let c = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let d = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let cd = c.dot(d);

        let e = Vector4f::new(1.0, 2.0, -2.0, 3.0);
        let f = Vector4f::new(3.0, 4.0, 1.0, 1.5);
        let ef = e.dot(f);

        assert_eq!(ab, 0.0);
        assert_eq!(cd, 1.0);
        assert_eq!(ef, 13.5);
    }

    #[test]
    pub fn test_normalize(){
        let a = Vector4f::new(100.0, 0.0, 0.0, 0.0);
        let b = Vector4f::new(0.0, 100.0, 0.0, 0.0);
        let _c = Vector4f::new(1.0, 1.0, 1.0, 1.0);

        let an = Vector4f::new(1.0, 0.0, 0.0, 0.0);
        let bn = Vector4f::new(0.0, 1.0, 0.0, 0.0);
        let cn = Vector4f::new(0.707107, 0.707107, 0.707107, 0.707107);

        assert_eq!(an, a.normalize());
        assert_eq!(bn, b.normalize());
        assert!((cn.x - 0.707107).abs() < EPS);
        assert!((cn.y - 0.707107).abs() < EPS);
        assert!((cn.z - 0.707107).abs() < EPS);
        assert!((cn.w - 0.707107).abs() < EPS);
    }
}