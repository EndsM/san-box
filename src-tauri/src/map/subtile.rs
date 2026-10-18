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
}
