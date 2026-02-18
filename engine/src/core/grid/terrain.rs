use super::super::geom::{Delta};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    Ground,
    Void,   
    WaterStill,
    WaterCurrent(Delta)  // On turn end, units are pushed by delta
}

impl TerrainType {
    // Check if the terrain type is walkable
    pub fn is_walkable(&self) -> bool {
        match self {
            TerrainType::Ground => true,
            TerrainType::Void => false,
            TerrainType::WaterStill => true,
            TerrainType::WaterCurrent(_) => true,
        }
    }

    pub fn get_cost(&self) -> i16 {
        match self {
            TerrainType::Ground => 1,
            TerrainType::Void => i16::MAX,
            TerrainType::WaterStill => 2,
            TerrainType::WaterCurrent(_) => 2,
        }
    }
}
