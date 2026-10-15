use serde::{Deserialize, Serialize};

use crate::map::id::RegionId;

// Use a enum for terrains for now
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Terrain {
    #[default]
    Plain,
    Grassland,
    Forest,
    Hill,
    Mountain,
    Desert,
    Swamp,
    Lake,
    River,
    Sea,
}

impl Terrain {
    pub const fn is_water(self) -> bool {
        matches!(self, Terrain::Lake | Terrain::Sea | Terrain::River)
    }
}

// Tile could be on a road
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RoadLevel {
    #[default]
    None,
    Path,
    Road,
    Highway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MapTile {
    terrain: Terrain,
    // Maybe add a elevation to tiles
    elevation: u8,
    road: RoadLevel,
    region: Option<RegionId>,
}

impl MapTile {
    pub const fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            elevation: 0,
            road: RoadLevel::None,
            region: None,
        }
    }
}
