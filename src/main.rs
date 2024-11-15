#![feature(int_roundings)]
mod grid;
mod tests;

use grid::{AxialPos, CubePos};

fn main() {
    let x = CubePos::default();
    let b = CubePos::new(100, -50, -50);
    let z = AxialPos::new(20, 20);
    let y: CubePos = z.into();
    let l: AxialPos = y.into();
    println!("X: {}", x);
    println!("B: {}", b);
    println!("X - B = C: {}", x - b);
    println!("Z: {}", z);
    println!("Y: {}", y);
    println!("L: {}", l);
}
