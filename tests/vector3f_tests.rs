#[cfg(test)]
mod tests {
    use ember_math::core::Vector3f;

    static EPS:f32 = 0.0000001;

    #[test]
    pub fn test_create_vector3f(){
        let a = Vector3f::new(1.0, 1.0, 1.0);
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 1.0);
        assert_eq!(a.z, 1.0);
    }

    #[test]
    pub fn test_equality(){
        let a = Vector3f::new(-1.0, 1.0, 3.0);
        let b = Vector3f::new(-1.0, 1.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    pub fn test_magnitude(){
        let a = Vector3f::new(1.0, 0.0, 0.0);
        let b = Vector3f::new(1.0, 1.0, 1.0);
        assert!((b.magnitude() - 1.7320508).abs() < EPS);
        assert_eq!(1.0, a.magnitude());
    }

    #[test]
    pub fn test_magnitude_squared(){
        let a = Vector3f::new(3.0, 4.0, 1.0);
        let b = Vector3f::new(-1.0, -2.0, 2.0);
        let c = Vector3f::new(0.0, 0.0, 0.0);
        assert_eq!(26.0, a.magnitude_squared());
        assert_eq!(9.0, b.magnitude_squared());
        assert_eq!(0.0, c.magnitude_squared());
    }

    #[test]
    pub fn test_one(){
        let a = Vector3f::new(1.0, 1.0, 1.0);
        assert_eq!(a, Vector3f::one());
    }

    #[test]
    pub fn test_zero(){
        let a = Vector3f::new(0.0, 0.0, 0.0);
        assert_eq!(Vector3f::zero(), a);
    }

    #[test]
    pub fn test_dot(){
        let a = Vector3f::new(1.0, 0.0, 0.0);
        let b = Vector3f::new(0.0, 1.0, 0.0);
        let ab = a.dot(b);

        let c = Vector3f::new(1.0, 0.0, 0.0);
        let d = Vector3f::new(1.0, 0.0, 0.0);
        let cd = c.dot(d);

        let e = Vector3f::new(1.0, 2.0, -2.0);
        let f = Vector3f::new(3.0, 4.0, 1.0);
        let ef = e.dot(f);

        assert_eq!(ab, 0.0);
        assert_eq!(cd, 1.0);
        assert_eq!(ef, 9.0);

    }

    #[test]
    pub fn test_cross(){
        let a = Vector3f::new(2.0, 3.0, 4.0);
        let b = Vector3f::new(5.0, 6.0, 7.0);
        let r = Vector3f::new(-3.0, 6.0, -3.0);
        assert_eq!(a.cross(b), r);
    }

    #[test]
    pub fn test_normalize(){
        let a = Vector3f::new(100.0, 0.0, 0.0);
        let b = Vector3f::new(0.0, 100.0, 0.0);
        let c = Vector3f::new(1.0, 1.0, 1.0);

        let an = Vector3f::new(1.0, 0.0, 0.0);
        let bn = Vector3f::new(0.0, 1.0, 0.0);
        let cn = Vector3f::new(0.707107, 0.707107, 0.707107);

        assert_eq!(an, a.normalize());
        assert_eq!(bn, b.normalize());
        assert!((cn.x - 0.707107).abs() < EPS);
        assert!((cn.y - 0.707107).abs() < EPS);
        assert!((cn.z - 0.707107).abs() < EPS);
    }

    #[test]
    pub fn test_add(){
        let a = Vector3f::new(1.0, -1.0, 1.0);
        let b = Vector3f::new(0.01, 0.01, 0.01);
        let c = a + b;
        assert_eq!(c.x, 1.01);
        assert_eq!(c.y, -0.99);
        assert_eq!(c.z, 1.01);
    }

    #[test]
    pub fn test_sub(){
        let a = Vector3f::one();
        let b = Vector3f::new(0.5, -0.5, 0.1);
        let c = a - b;
        assert_eq!(c.x, 0.5);
        assert_eq!(c.y, 1.5);
        assert_eq!(c.z, 0.9);
    }
}