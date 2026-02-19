use super::super::geom::{Position};
use super::super::unit::{UnitId};
use super::TerrainType;

#[derive(Clone)]
pub struct Grid {
    width: usize,
    height: usize,

    terrain: Vec<TerrainType>,
    heightmap: Vec<u8>,
    occupancy: Vec<Option<UnitId>>,
}

impl Grid {
    /// Convert (x, y) to index in flattened Vec
    #[inline]
    fn idx(&self, pos: Position) -> usize {
        debug_assert!(self.in_bounds(pos), "Position out of bounds: {:?}", pos);
        pos.y() * self.width + pos.x()
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    #[inline]
    fn in_bounds(&self, pos: Position) -> bool {
        pos.x() < self.width && pos.y() < self.height
    }
    
    /// Constructor for a new grid
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;

        Grid {
            width,
            height,
            terrain: vec![TerrainType::Ground; size],
            heightmap: vec![0; size],
            occupancy: vec![None; size],
        }
    }

    pub fn get_terrain_type(&self, pos: Position) -> Option<&TerrainType> {
        debug_assert!(self.in_bounds(pos), "get_terrain_type called with out-of-bounds position: {:?}", pos);
        
        if self.in_bounds(pos) {
            Some(&self.terrain[self.idx(pos)])
        } else {
            None
        }
    }

    pub fn set_occupancy(&mut self, pos: Position, unit: Option<UnitId>) {
        debug_assert!(self.in_bounds(pos), "set_occupancy called with out-of-bounds position: {:?}", pos);

        if self.in_bounds(pos) {
            let idx = self.idx(pos);
            self.occupancy[idx] = unit;
        }
    }

    pub fn set_terrain(&mut self, pos: Position, terrain: TerrainType) {
        debug_assert!(self.in_bounds(pos), "set_occupancy called with out-of-bounds position: {:?}", pos);
        
        if self.in_bounds(pos) {
            let idx = self.idx(pos);
            self.terrain[idx] = terrain;
        }
    }
}
