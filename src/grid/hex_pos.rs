use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    ops,
};

pub trait Pos {}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct AxialPos {
    pub q: f64,
    pub r: f64,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct CubePos {
    pub q: f64,
    pub r: f64,
    pub s: f64,
}

impl CubePos {
    pub fn new(q: f64, r: f64, s: f64) -> Self {
        assert_eq!(q + r + s, 0.0);
        Self { q, r, s }
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

impl Eq for CubePos {}

impl Hash for CubePos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        format!("{} {} {}", self.q, self.r, self.s).hash(state);
    }
}

impl Pos for CubePos {}

impl From<AxialPos> for CubePos {
    fn from(item: AxialPos) -> Self {
        Self {
            q: item.q,
            r: item.r,
            s: -item.q - item.r,
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
        let val: CubePos = *self + rhs;
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
    pub fn new(q: f64, r: f64) -> Self {
        Self { q, r }
    }
}

impl Display for AxialPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[ Q: {}, R: {} ]", self.q, self.r))
    }
}

impl Eq for AxialPos {}

impl Hash for AxialPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{} {}", self.q, self.r).hash(state);
    }
}

impl From<CubePos> for AxialPos {
    fn from(item: CubePos) -> Self {
        Self {
            q: item.q,
            r: item.r,
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
        Directions::North => CubePos::new(0.0, -1.0, 1.0),
        Directions::NorthEast => CubePos::new(1.0, -1.0, 0.0),
        Directions::SouthEast => CubePos::new(1.0, 0.0, -1.0),
        Directions::South => CubePos::new(0.0, 1.0, -1.0),
        Directions::SouthWest => CubePos::new(-1.0, 1.0, 0.0),
        Directions::NorthWest => CubePos::new(-1.0, 0.0, 1.0),
    }
}

pub fn cube_diagonal_dir(dir: Directions) -> CubePos {
    match dir {
        Directions::North => CubePos::new(1.0, -2.0, 1.0),
        Directions::NorthEast => CubePos::new(2.0, -1.0, -1.0),
        Directions::SouthEast => CubePos::new(1.0, 1.0, -2.0),
        Directions::South => CubePos::new(-1.0, -1.0, 2.0),
        Directions::SouthWest => CubePos::new(-2.0, 1.0, 1.0),
        Directions::NorthWest => CubePos::new(-1.0, -1.0, 2.0),
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
        Directions::North => AxialPos::new(0.0, -1.0),
        Directions::NorthEast => AxialPos::new(1.0, -1.0),
        Directions::SouthEast => AxialPos::new(1.0, 0.0),
        Directions::South => AxialPos::new(0.0, -1.0),
        Directions::SouthWest => AxialPos::new(-1.0, 1.0),
        Directions::NorthWest => AxialPos::new(-1.0, 0.0),
    }
}

pub fn axial_neighbor(orig: AxialPos, dir: Directions) -> AxialPos {
    orig + axial_dir(dir)
}

pub struct OffsetPos {
    pub x: i64,
    pub y: i64,
}

pub enum Offset {
    Odd(OffsetPos),
    Even(OffsetPos),
}

pub enum OffsetCoord {
    Pointy(Offset),
    Flat(Offset),
}

pub fn axial_from_offset(val: OffsetCoord) -> AxialPos {
    match val {
        OffsetCoord::Pointy(offset) => match offset {
            Offset::Odd(offset_pos) => {
                let q = offset_pos.x - (offset_pos.y - (offset_pos.y) & 1) / 2;
                let r = offset_pos.y;
                AxialPos::new(q as f64, r as f64)
            }
            Offset::Even(offset_pos) => {
                let q = offset_pos.x - (offset_pos.y + (offset_pos.y) & 1) / 2;
                let r = offset_pos.y;
                AxialPos::new(q as f64, r as f64)
            }
        },
        OffsetCoord::Flat(offset) => match offset {
            Offset::Odd(offset_pos) => {
                let q = offset_pos.x;
                let r = offset_pos.y - (offset_pos.x - (offset_pos.x) & 1) / 2;
                AxialPos::new(q as f64, r as f64)
            }
            Offset::Even(offset_pos) => {
                let q = offset_pos.x;
                let r = offset_pos.y - (offset_pos.x + (offset_pos.x) & 1) / 2;
                AxialPos::new(q as f64, r as f64)
            }
        },
    }
}

pub fn axial_to_offset(pos: AxialPos, val: OffsetCoord) -> OffsetCoord {
    let res = match val {
        OffsetCoord::Pointy(offset) => match offset {
            Offset::Odd(offset_pos) => {
                let q = pos.q as i64;
                let r = pos.r as i64;
                let x = q + (r - r & 1) / 2;
                let y = r;
                OffsetCoord::Pointy(Offset::Odd(OffsetPos { x, y }))
            }
            Offset::Even(offset_pos) => {
                let q = pos.q as i64;
                let r = pos.r as i64;
                let x = q + (r + r & 1) / 2;
                let y = r;
                OffsetCoord::Pointy(Offset::Even(OffsetPos { x, y }))
            }
        },
        OffsetCoord::Flat(offset) => match offset {
            Offset::Odd(offset_pos) => {
                let q = pos.q as i64;
                let r = pos.r as i64;
                let x = q;
                let y = r + (q - q & 1) / 2;
                OffsetCoord::Flat(Offset::Odd(OffsetPos { x, y }))
            }
            Offset::Even(offset_pos) => {
                let q = pos.q as i64;
                let r = pos.r as i64;
                let x = q;
                let y = r + (q + q & 1) / 2;
                OffsetCoord::Flat(Offset::Even(OffsetPos { x, y }))
            }
        },
    };
    res
}
