use std::{
    collections::{hash_map::Values, HashMap},
    ops::{Div, Index, IndexMut},
};

use super::CubePos;

/// Hex tile data.
#[derive(Debug, Default, Clone, Copy)]
pub struct Data {
    /// Whether the tile can be moved onto to.
    pub traversible: bool,
    /// Whether the tile has water.
    pub water: bool,
    /// Elevation from noise algorithms.
    pub elevation: f64,
    /// Terrain cost for path finding algorithms.
    pub terrain: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum GridShape {
    Rectangle,
    RectanglePointy,
    Hexagon,
    Rhombus,
}

impl Default for GridShape {
    fn default() -> Self {
        Self::Rectangle
    }
}

///
/// <summary>
/// Contains a HashSet of tiles and a shape of the map.
/// </summary>
///
#[derive(Debug, Default)]
pub struct Map {
    pub shape: GridShape,
    pub tiles: HashMap<CubePos, Data>,
    pub dimensions: (i64, i64),
}

impl Map {
    ///
    /// <summary>
    /// Initialize a new <typeref = "Map">Map</typeref> object.
    /// </summary>
    ///
    pub fn new(shape: GridShape, width: i64, height: i64) -> Self {
        //  Initialize with default values.
        let capacity = width * height + 1;
        //  Initialize the tile map to the correct capacity.
        let mut map = Map {
            shape: shape.clone(),
            tiles: HashMap::with_capacity(capacity as usize),
            dimensions: (width, height),
        };
        //  Fill in the tile map as necessary.
        match shape {
            GridShape::Rectangle => Self::init_flat_rect(&mut map),
            GridShape::RectanglePointy => Self::init_pointy_rect(&mut map),
            GridShape::Hexagon => todo!(),
            GridShape::Rhombus => todo!(),
        };
        map
    }

    fn get(&self, pos: CubePos) -> Option<&Data> {
        self.tiles.get(&pos)
    }

    fn get_mut(&mut self, pos: CubePos) -> Option<&mut Data> {
        self.tiles.get_mut(&pos)
    }

    fn init_pointy_rect(map: &mut Map) -> () {
        let left = 0;
        let right = map.dimensions.0;
        let top = 0;
        let bottom = map.dimensions.1;
        // Column iteration
        for r in top..bottom {
            //  Row iteration
            let r_offset = (r >> 1) as i64;
            let start = left - r_offset;
            let end = right - r_offset;
            for q in start..end {
                let pos = CubePos::new(q as f64, r as f64, (-q - r as i64) as f64);
                map.tiles.insert(pos, Data::default());
            }
        }
    }

    fn init_flat_rect(map: &mut Map) -> () {
        let left: i64 = 0;
        let right = map.dimensions.0;
        let top: i64 = 0;
        let bottom = map.dimensions.1;
        for q in left..right {
            let q_offset = q >> 1;
            let start = top - q_offset;
            let end = bottom - q_offset;
            for r in start..end {
                let pos = CubePos::new(q as f64, r as f64, (-q - r as i64) as f64);
                map.tiles.insert(pos, Data::default());
            }
        }
    }
}

impl Index<CubePos> for Map {
    type Output = Data;

    fn index(&self, index: CubePos) -> &Self::Output {
        if (index.q > self.dimensions.0 as f64 || index.q < 0.0)
            && (index.r > self.dimensions.1 as f64 || index.r < 0.0)
        {
            panic!("Index out of bounds.")
        }
        self.get(index).unwrap()
    }
}

impl IndexMut<CubePos> for Map {
    fn index_mut(&mut self, index: CubePos) -> &mut Self::Output {
        if (index.q > self.dimensions.0 as f64 || index.q < 0.0)
            && (index.r > self.dimensions.1 as f64 || index.r < 0.0)
        {
            panic!("Index out of bounds.")
        }
        self.get_mut(index).unwrap()
    }
}
