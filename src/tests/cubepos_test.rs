#[cfg(test)]
mod basic {
    use crate::hex_pos::{AxialPos, CubePos};

    #[test]
    fn cube_equality() {
        let a = CubePos::new(1.0, 1.0, -2.0);
        let b = CubePos::new(1.0, 1.0, -2.0);
        let c = CubePos::default();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn cube_addition() {
        let a = CubePos::new(1.0, 1.0, -2.0);
        let b = CubePos::new(1.0, 2.0, -3.0);
        let c = CubePos::new(2.0, 3.0, -5.0);
        assert_eq!(a + b, c);
    }

    #[test]
    fn cube_subtraction() {
        let a = CubePos::new(1.0, 1.0, -2.0);
        let b = CubePos::new(1.0, 2.0, -3.0);
        let c = CubePos::new(0.0, -1.0, 1.0);
        assert_eq!(a - b, c);
    }

    #[test]
    fn cube_add_assign() {
        let mut a = CubePos::new(1.0, 1.0, -2.0);
        let b = CubePos::new(0.0, -1.0, 1.0);
        let c = CubePos::new(1.0, 0.0, -1.0);
        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn cube_sub_assign() {
        let mut a = CubePos::new(1.0, 1.0, -2.0);
        let b = CubePos::new(0.0, -1.0, 1.0);
        let c = CubePos::new(1.0, 2.0, -3.0);
        a -= b;
        assert_eq!(a, c);
    }

    #[test]
    fn cube_to_axial() {
        let a = CubePos::new(1.0, 1.0, -2.0);
        let b: AxialPos = a.into();
        let c = AxialPos::new(1.0, 1.0);
        assert_eq!(b, c);
    }

    #[test]
    fn axial_to_cube() {
        let a = AxialPos::new(1.0, 1.0);
        let b = CubePos::new(1.0, 1.0, -2.0);
        let c: CubePos = a.into();
        assert_eq!(b, c);
    }

    #[test]
    fn axial_addition() {
        let a = AxialPos::new(10.0, 10.0);
        let b = AxialPos::new(-5.0, 5.0);
        let c = a + b;
        assert_eq!(c, AxialPos { q: 5.0, r: 15.0 });
    }

    #[test]
    fn axial_subtraction() {
        let a = AxialPos::new(10.0, 10.0);
        let b = AxialPos::new(-5.0, 5.0);
        let c = a - b;
        assert_eq!(c, AxialPos { q: 15.0, r: 5.0 });
    }
}
