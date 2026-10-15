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

    pub const fn with_elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation;
        self
    }

    pub const fn with_road(mut self, road: RoadLevel) -> Self {
        self.road = road;
        self
    }

    pub const fn with_region(mut self, region: RegionId) -> Self {
        self.region = Some(region);
        self
    }

    pub const fn terrain(&self) -> Terrain {
        self.terrain
    }

    pub const fn elevation(&self) -> u8 {
        self.elevation
    }

    pub const fn road(&self) -> RoadLevel {
        self.road
    }

    pub const fn region(&self) -> Option<RegionId> {
        self.region
    }

    pub const fn is_water(&self) -> bool {
        self.terrain.is_water()
    }

    pub fn set_terrain(&mut self, terrain: Terrain) {
        self.terrain = terrain;
    }

    pub fn set_elevation(&mut self, elevation: u8) {
        self.elevation = elevation;
    }

    pub fn set_road(&mut self, road: RoadLevel) {
        self.road = road;
    }

    pub fn set_region(&mut self, region: Option<RegionId>) {
        self.region = region;
    }
}
