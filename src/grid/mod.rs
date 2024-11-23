#![allow(unused)]
use std::ops::Div;

use lazy_static::lazy_static;

pub mod hex_pos;
pub mod map;

pub use crate::grid::hex_pos::{AxialPos, CubePos, Directions};
pub use crate::grid::map::{Data, GridShape, Map};

pub enum HexOrientation {
    Pointy,
    Flat,
}

lazy_static! {
    static ref SQRT_3: f64 = f64::sqrt(3.0);
    static ref PI: f64 = 3.14159265358979;
}

pub struct Matrix(f64, f64, f64, f64);

pub struct Orientation {
    pub start_angle: f64,
    pub to_screen: Matrix,
    pub to_grid: Matrix,
}

impl Orientation {
    pub fn new(vals: (f64, f64, f64, f64, f64, f64, f64, f64), start_angle: f64) -> Self {
        Self {
            start_angle,
            to_screen: Matrix {
                0: vals.0,
                1: vals.1,
                2: vals.2,
                3: vals.3,
            },
            to_grid: Matrix {
                0: vals.4,
                1: vals.5,
                2: vals.6,
                3: vals.7,
            },
        }
    }
}

lazy_static! {
    static ref POINTY: Orientation = Orientation::new(
        (
            *SQRT_3,
            (*SQRT_3) / 2.0,
            0.0,
            3.0 / 2.0,
            *SQRT_3 / 3.0,
            -1.0 / 3.0,
            0.0,
            (2.0 / 3.0)
        ),
        0.5
    );
    static ref FLAT: Orientation = Orientation::new(
        (
            3.0 / 2.0,
            0.0,
            *SQRT_3 / 2.0,
            *SQRT_3,
            2.0 / 3.0,
            0.0,
            -1.0 / 3.0,
            *SQRT_3 / 3.0
        ),
        0.0
    );
}

#[derive(Default, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Stores a map layout relative to a point.
pub struct Layout {
    /// Orientation: POINTY or FLAT.
    pub orientation: Orientation,
    /// Hexagon size.
    pub size: (f64, f64),
    /// Center of layout.
    pub origin: Point,
}

impl Layout {
    pub fn new(orientation: Orientation, size: (f64, f64), origin: Point) -> Self {
        Self {
            orientation,
            size,
            origin,
        }
    }

    pub fn hex_to_pixel(&self, hex: AxialPos) -> Point {
        let m = &self.orientation.to_screen;
        let s = self.size;
        let origin = &self.origin;
        let x = (m.0 * (hex.q as f64) + m.1 * (hex.r as f64)) * (s.0 as f64);
        let y = (m.2 * (hex.q as f64) + m.3 * (hex.r as f64)) * (s.1 as f64);
        Point::new(x + origin.x, y + origin.y)
    }

    pub fn pixel_to_hex(&self, p: Point) -> CubePos {
        let pt = Point {
            x: (p.x - self.origin.x),
            y: (p.y - self.origin.y),
        };
        let m = &self.orientation.to_grid;
        let size = self.size;
        let q = m.0 * pt.x + m.1 * pt.y / size.0;
        let r = m.2 * pt.x + m.3 * pt.y / size.1;
        CubePos {
            q: (q as f64),
            r: (r as f64),
            s: ((-q - r) as f64),
        }
    }
}

pub fn hex_corner_offset(layout: &Layout, corner: i32) -> Point {
    let size = layout.size;
    let angle = 2.0 * *PI * (layout.orientation.start_angle + f64::from(corner)).div(6.0);
    Point {
        x: (size.0) * f64::cos(angle),
        y: (size.1) * f64::sin(angle),
    }
}

pub fn hex_polygon_corners(layout: &Layout, hex: CubePos) -> Vec<Point> {
    let center = layout.hex_to_pixel(hex.into());
    let mut corners: Vec<Point> = Vec::with_capacity(7);
    for i in 0..6 {
        let offset = hex_corner_offset(layout, i);
        let p = Point {
            x: center.x + offset.x,
            y: center.y + offset.y,
        };
        corners.push(p);
    }
    corners
}
