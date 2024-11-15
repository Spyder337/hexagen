mod grid;
mod tests;

use crate::grid::OffsetPos;
fn main() {
    let x = OffsetPos::default();
    let y: u8 = 1;
    let z: u8 = 3;
    println!("{}", format!("Byte value: {z:b}"));
    println!("Hello, world!");
}
