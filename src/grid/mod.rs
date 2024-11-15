use std::ops;

/// <summary>
/// Alignment type for the offset grid.
/// </summary>
pub enum OffsetType {
    OddR = 0b0001,  //  Odd row layout
    EvenR = 0b0010, //  Even row layout
    OddQ = 0b0100,  //  Odd column layout
    EvenQ = 0b1000, //  Even column layout
}

pub struct OffsetPos {
    pub q: i64,
    pub r: i64,
}

pub struct AxialPos {
    pub q: i64,
    pub r: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CubePos {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

pub enum DoubleType {
    Height = 0b01, // The height doubles every other tile.
    Width = 0b10,  // The width doubles every other tile.
}

pub struct DoublePos {}

impl OffsetPos {
    pub fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }
}

impl Default for OffsetPos {
    fn default() -> Self {
        Self { q: 0, r: 0 }
    }
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
