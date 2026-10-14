// The sub tile is within a world tile
pub const SUB_TILE_SIZE: usize = 15;
// Calculate the result count here for now
pub const SUB_TILE_COUNT: usize = SUB_TILE_SIZE * SUB_TILE_SIZE;

pub const SIZE_I32: i32 = SUB_TILE_SIZE as i32;

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

impl Direction {
    // Cardinal directions only
    pub const CARDINAL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    // All directions, clockwise from North
    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];
}

// This represent a tile's coord on the big map
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    pub const ORIGIN: TileCoord = TileCoord { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Translate to one subtile's Global Coord
    pub const fn sub(self, local: LocalCoord) -> SubTileCoord {
        SubTileCoord::new(
            self.x * SIZE_I32 + local.x as i32,
            self.y * SIZE_I32 + local.y as i32,
        )
    }
}

// This represent a subtile's coord in its Tile, thus LocalCoord
pub struct LocalCoord {
    x: u8,
    y: u8,
}

impl LocalCoord {
    pub const CENTER: LocalCoord = LocalCoord {
        x: (SUB_TILE_SIZE / 2) as u8,
        y: (SUB_TILE_SIZE / 2) as u8,
    };
}

// This represent a subtile's global Coord, which means a subtile now can be traced globally.
pub struct SubTileCoord {
    pub x: i32,
    pub y: i32,
}

impl SubTileCoord {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn tile(self) -> TileCoord {
        TileCoord::new(self.x.div_euclid(SIZE_I32), self.y.div_euclid(SIZE_I32))
    }
}

// Represent subtile's local coord in a battle map
pub struct BattleCoord {
    x: i32,
    y: i32,
}
