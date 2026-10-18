use serde::{Deserialize, Serialize};

use crate::map::Terrain;

// A list of things could be built on a tile, this work as a placeholder for now.
// As in the future, may create speciallized buildings data structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Structure {
    #[default]
    None,
    Road,
    Wall,
    Gate,
    Building,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SubTile {
    terrain: Terrain,
    elevation: u8,
    structure: Structure,
}

impl SubTile {
    // default to nothing basically
    pub const fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            elevation: 0,
            structure: Structure::None,
        }
    }

    pub const fn with_elevation(mut self, elevation: u8) -> Self {
        self.elevation = elevation;
        self
    }

    pub const fn with_structure(mut self, structure: Structure) -> Self {
        self.structure = structure;
        self
    }

    pub const fn terrain(&self) -> Terrain {
        self.terrain
    }

    pub const fn elevation(&self) -> u8 {
        self.elevation
    }

    pub const fn structure(&self) -> Structure {
        self.structure
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

    pub fn set_structure(&mut self, structure: Structure) {
        self.structure = structure;
    }
}
