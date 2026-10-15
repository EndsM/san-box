use crate::map::{MapTile, TileRect};

#[derive(Debug, Clone)]
pub struct WorldMap {
    bounds: TileRect,
    tiles: Vec<MapTile>,
}

impl WorldMap {
    pub fn new(bounds: TileRect) -> Self {
        let count = bounds.width() as usize * bounds.height() as usize;
        Self {
            bounds,
            tiles: vec![MapTile::default(); count],
        }
    }
}
