use serde::{Deserialize, Serialize};

use crate::map::{SUB_TILE_COUNT, subtile::SubTile};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubMap {
    tiles: Vec<SubTile>,
}

impl SubMap {
    pub fn filled(tile: SubTile) -> Self {
        Self {
            tiles: vec![tile; SUB_TILE_COUNT],
        }
    }
}

impl Default for SubMap {
    fn default() -> Self {
        Self::filled(SubTile::default())
    }
}
