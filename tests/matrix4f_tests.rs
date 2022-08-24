#[cfg(test)]
mod tests {
    use ember_math::core::Matrix4f;
    

    static EPS:f32 = 1e-5;

    #[test]
    pub fn test_identity(){
        let m = Matrix4f::identity();

        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[1], 0.0);
        assert_eq!(m.data[2], 0.0);
        assert_eq!(m.data[3], 0.0);

        assert_eq!(m.data[4], 0.0);
        assert_eq!(m.data[5], 1.0);
        assert_eq!(m.data[6], 0.0);
        assert_eq!(m.data[7], 0.0);

        assert_eq!(m.data[8], 0.0);
        assert_eq!(m.data[9], 0.0);
        assert_eq!(m.data[10], 1.0);
        assert_eq!(m.data[11], 0.0);

        assert_eq!(m.data[12], 0.0);
        assert_eq!(m.data[13], 0.0);
        assert_eq!(m.data[14], 0.0);
        assert_eq!(m.data[15], 1.0);
    }
}