use crate::map::{SUB_TILE_COUNT, subtile::SubTile};

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
