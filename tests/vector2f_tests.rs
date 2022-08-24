#[cfg(test)]
mod tests {
    use ember_math::core::Vector2f;

    static EPS:f32 = 0.0000001;
    static PI:f32 = std::f32::consts::PI;
    static HALF_PI:f32 = PI / 2.0;

    #[test]
    pub fn test_create_vector2f(){
        let a = Vector2f::new(1.0, 1.0);
        assert_eq!(a.x, 1.0);
        assert_eq!(a.y, 1.0);
    }

    #[test]
    pub fn test_vector2f_equality(){
        let a = Vector2f::new(-1.0, 1.0);
        let b = Vector2f::new(-1.0, 1.0);
        assert_eq!(a, b);
    }

    #[test]
    pub fn test_vector2f_magnitude(){
        let a = Vector2f::new(1.0, 0.0);
        let b = Vector2f::new(1.0, 1.0);
        assert!((b.magnitude() - 1.4142135).abs() < EPS);
        assert_eq!(1.0, a.magnitude());
    }

    #[test]
    pub fn test_vector2f_magnitude_squared(){
        let a = Vector2f::new(3.0, 4.0);
        let b = Vector2f::new(-1.0, -2.0);
        let c = Vector2f::new(0.0, 0.0);
        assert_eq!(25.0, a.magnitude_squared());
        assert_eq!(5.0, b.magnitude_squared());
        assert_eq!(0.0, c.magnitude_squared());
    }

    #[test]
    pub fn test_vector2f_one(){
        let a = Vector2f::new(1.0, 1.0);
        assert_eq!(a, Vector2f::one());
    }

    #[test]
    pub fn test_vector2f_zero(){
        let a = Vector2f::new(0.0, 0.0);
        assert_eq!(Vector2f::zero(), a);
    }

    #[test]
    pub fn test_vector2f_dot(){
        let a = Vector2f::new(1.0, 0.0);
        let b = Vector2f::new(0.0, 1.0);
        let ab = a.dot(b);

        let c = Vector2f::new(1.0, 0.0);
        let d = Vector2f::new(1.0, 0.0);
        let cd = c.dot(d);

        let e = Vector2f::new(1.0, 2.0);
        let f = Vector2f::new(3.0, 4.0);
        let ef = e.dot(f);

        assert_eq!(ab, 0.0);
        assert_eq!(cd, 1.0);
        assert_eq!(ef, 11.0);

    }

    #[test]
    pub fn test_normalize(){
        let a = Vector2f::new(100.0, 0.0);
        let b = Vector2f::new(0.0, 100.0);
        let _c = Vector2f::new(1.0, 1.0);

        let an = Vector2f::new(1.0, 0.0);
        let bn = Vector2f::new(0.0, 1.0);
        let cn = Vector2f::new(0.707107, 0.707107);

        assert_eq!(an, a.normalize());
        assert_eq!(bn, b.normalize());
        assert!((cn.x - 0.707107).abs() < EPS);
        assert!((cn.y - 0.707107).abs() < EPS);

    }

    #[test]
    pub fn test_angle_between(){
        let a = Vector2f::new(1.0, 0.0);
        let b = Vector2f::new(0.0, 1.0);
        let ab = a.angle_between_rad(b);

        assert!((ab - HALF_PI).abs() < EPS);
    }

    #[test]
    pub fn test_add(){
        let a = Vector2f::new(1.0, -1.0);
        let b = Vector2f::new(-1.0, 1.0);
        assert_eq!(a+b, Vector2f::new(0.0, 0.0));
    }
}