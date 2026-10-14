// The sub tile is within a world tile
pub const SUB_TILE_SIZE: usize = 15;
// Calculate the result count here for now
pub const SUB_TILE_COUNT: usize = SUB_TILE_SIZE * SUB_TILE_SIZE;

// Directions may used later for unit facing, pathfinding, etc
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {}

// This represent a tile's coord on the big map
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    pub const ORIGIN: TileCoord = TileCoord { x: 0, y: 0 };
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
