// The sub tile is within a world tile
pub const SUB_TILE_SIZE: usize = 15;
// Calculate the result count here for now
pub const SUB_TILE_COUNT: usize = SUB_TILE_SIZE * SUB_TILE_SIZE;

pub struct WorldCoord {
    pub x: i32,
    pub y: i32,
}

impl WorldCoord {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

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
