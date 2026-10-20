use serde::{Deserialize, Serialize};

use crate::map::{LocalCoord, SUB_TILE_COUNT, subtile::SubTile};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Vec<SubTile>", into = "Vec<SubTile>")]
pub struct SubMap {
    tiles: Vec<SubTile>,
}

impl SubMap {
    pub fn filled(tile: SubTile) -> Self {
        Self {
            tiles: vec![tile; SUB_TILE_COUNT],
        }
    }

    pub fn get(&self, at: LocalCoord) -> &SubTile {
        &self.tiles[at.index()]
    }

    pub fn get_mut(&mut self, at: LocalCoord) -> &mut SubTile {
        &mut self.tiles[at.index()]
    }

    pub fn set(&mut self, at: LocalCoord, tile: SubTile) {
        self.tiles[at.index()] = tile;
    }

    // Row-major over the whole grid
    pub fn iter(&self) -> impl Iterator<Item = (LocalCoord, &SubTile)> {
        LocalCoord::all().zip(self.tiles.iter())
    }
}

impl Default for SubMap {
    fn default() -> Self {
        Self::filled(SubTile::default())
    }
}

impl From<SubMap> for Vec<SubTile> {
    fn from(map: SubMap) -> Self {
        map.tiles
    }
}

impl TryFrom<Vec<SubTile>> for SubMap {
    type Error = String;

    fn try_from(tiles: Vec<SubTile>) -> Result<Self, Self::Error> {
        if tiles.len() == SUB_TILE_COUNT {
            Ok(Self { tiles })
        } else {
            Err(format!(
                "sub map needs {SUB_TILE_COUNT} sub tiles, got {}",
                tiles.len()
            ))
        }
    }
}
