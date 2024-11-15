use std::{
    collections::HashSet,
    fmt::{Display, Pointer},
    ops,
};

pub trait Pos {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AxialPos {
    pub q: i64,
    pub r: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directions {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CubePos {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

impl CubePos {
    pub fn new(q: i64, r: i64, s: i64) -> Self {
        assert_eq!(q + r + s, 0);
        Self { q, r, s }
    }
}

impl Default for CubePos {
    fn default() -> Self {
        Self { q: 0, r: 0, s: 0 }
    }
}

impl Display for CubePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[ Q: {}, R: {}, S: {} ]",
            self.q, self.r, self.s
        ))
    }
}

impl Pos for CubePos {}

impl Into<AxialPos> for CubePos {
    fn into(self) -> AxialPos {
        AxialPos {
            q: self.q,
            r: self.r,
        }
    }
}

impl ops::Add for CubePos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl ops::AddAssign for CubePos {
    fn add_assign(&mut self, rhs: Self) {
        let val = *self + rhs;
        *self = val
    }
}

impl ops::Sub for CubePos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

impl ops::SubAssign for CubePos {
    fn sub_assign(&mut self, rhs: Self) {
        let val = *self - rhs;
        *self = val
    }
}

impl AxialPos {
    pub fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }
}

impl Default for AxialPos {
    fn default() -> Self {
        Self { q: 0, r: 0 }
    }
}

impl Display for AxialPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ Q: {}, R: {} ]", self.q, self.r))
    }
}

impl Into<CubePos> for AxialPos {
    fn into(self) -> CubePos {
        CubePos {
            q: self.q,
            r: self.r,
            s: -self.q - self.r,
        }
    }
}

impl Pos for AxialPos {}

impl ops::Add for AxialPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl ops::AddAssign for AxialPos {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl ops::Sub for AxialPos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

pub fn cube_dir(dir: Directions) -> CubePos {
    match dir {
        Directions::North => CubePos::new(0, -1, 1),
        Directions::NorthEast => CubePos::new(1, -1, 0),
        Directions::SouthEast => CubePos::new(1, 0, -1),
        Directions::South => CubePos::new(0, 1, -1),
        Directions::SouthWest => CubePos::new(-1, 1, 0),
        Directions::NorthWest => CubePos::new(-1, 0, 1),
    }
}

pub fn cube_diagonal_dir(dir: Directions) -> CubePos {
    match dir {
        Directions::North => CubePos::new(1, -2, 1),
        Directions::NorthEast => CubePos::new(2, -1, -1),
        Directions::SouthEast => CubePos::new(1, 1, -2),
        Directions::South => CubePos::new(-1, -1, 2),
        Directions::SouthWest => CubePos::new(-2, 1, 1),
        Directions::NorthWest => CubePos::new(-1, -1, 2),
    }
}

pub fn cube_neighbor(orig: CubePos, dir: Directions) -> CubePos {
    orig + cube_dir(dir)
}

pub fn cube_diag_neighbor(orig: CubePos, dir: Directions) -> CubePos {
    orig + cube_diagonal_dir(dir)
}

pub fn axial_dir(dir: Directions) -> AxialPos {
    match dir {
        Directions::North => AxialPos::new(0, -1),
        Directions::NorthEast => AxialPos::new(1, -1),
        Directions::SouthEast => AxialPos::new(1, 0),
        Directions::South => AxialPos::new(0, -1),
        Directions::SouthWest => AxialPos::new(-1, 1),
        Directions::NorthWest => AxialPos::new(-1, 0),
    }
}

pub fn axial_neighbor(orig: AxialPos, dir: Directions) -> AxialPos {
    orig + axial_dir(dir)
}

pub enum GridShape {
    Rectangle,
    Hexagon,
    Rhombus,
    PointyRectangle,
}

pub trait Grid {
    fn get_tiles() -> Vec<Box<dyn Pos>>;
}

pub struct GridHash {
    pub tiles: HashSet<AxialPos>,
}

pub struct Grid2D<'a> {
    pub tiles: &'a mut [Vec<AxialPos>],
    pub shape: GridShape,
}

impl<'a> Grid2D<'a> {
    pub fn pos_to_index(&'a self, pos: AxialPos) -> (usize, usize) {
        match self.shape {
            GridShape::Rectangle => {
                let x = pos.r as usize;
                let y = (pos.q + i64::div_floor(pos.r, 2)) as usize;
                return (x, y);
            }
            GridShape::Hexagon => {
                let x = pos.r as usize;
                // let y = (pos.q - i64::max(0)) as usize;
                // return (x, y);
                return (0, 0);
            }
            GridShape::Rhombus => (pos.r as usize, pos.q as usize),
            GridShape::PointyRectangle => todo!(),
        }
    }
}
