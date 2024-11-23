use lazy_static::lazy_static;

use crate::map::Map;
lazy_static! {
    static ref MAP_1: Map = Map::new(crate::map::GridShape::Rectangle, 128, 64);
}
#[cfg(test)]
pub mod maps {
    use super::MAP_1;
}
