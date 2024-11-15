#[cfg(test)]
mod basic {
    use crate::grid::CubePos;

    #[test]
    fn cube_equality() {
        let a = CubePos::new(1, 1, -2);
        let b = CubePos::new(1, 1, -2);
        let c = CubePos::default();

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn cube_addition() {
        let a = CubePos::new(1, 1, -2);
        let b = CubePos::new(1, 2, -3);
        let c = CubePos::new(2, 3, -5);
        assert_eq!(a + b, c);
    }

    #[test]
    fn cube_subtraction() {
        let a = CubePos::new(1, 1, -2);
        let b = CubePos::new(1, 2, -3);
        let c = CubePos::new(0, -1, 1);
        assert_eq!(a - b, c);
    }

    #[test]
    fn cube_add_assign() {
        let mut a = CubePos::new(1, 1, -2);
        let b = CubePos::new(0, -1, 1);
        let c = CubePos::new(1, 0, -1);
        a += b;
        assert_eq!(a, c);
    }

    #[test]
    fn cube_sub_assign() {
        let mut a = CubePos::new(1, 1, -2);
        let b = CubePos::new(0, -1, 1);
        let c = CubePos::new(1, 2, -3);
        a -= b;
        assert_eq!(a, c);
    }
}
