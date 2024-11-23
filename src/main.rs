#![feature(int_roundings)]
mod grid;
mod tests;

use grid::*;

fn main() {
    let x = CubePos::default();
    let b = CubePos::new(100.0, -50.0, -50.0);
    let z = AxialPos::new(20.0, 20.0);
    let y: CubePos = z.into();
    let l: AxialPos = y.into();
    let _shape = GridShape::Rectangle;
    let map = Map::new(_shape, 10, 6);
    println!("X: {}", x);
    println!("B: {}", b);
    println!("X - B = C: {}\n", x - b);
    println!("Z: {}", z);
    println!("Z into Cube is Y: {}", y);
    println!("Y into Axial is L: {}\n", l);
    println!("Map Dimensions : {:?}", map.dimensions);
    println!("Value at Tile : {}\n{:?}", x, map[x]);
}
