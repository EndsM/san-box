use std::hash::Hash;

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

    pub const fn delta(self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    pub const fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::NorthEast => Direction::SouthWest,
            Direction::East => Direction::West,
            Direction::SouthEast => Direction::NorthWest,
            Direction::South => Direction::North,
            Direction::SouthWest => Direction::NorthEast,
            Direction::West => Direction::East,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    // Check if the direction is diagonal
    pub const fn is_diagonal(self) -> bool {
        matches!(
            self,
            Direction::NorthEast
                | Direction::SouthEast
                | Direction::SouthWest
                | Direction::NorthWest
        )
    }
}

// The universal behavior for int based coords
pub trait GridCoord: Copy + Eq + Hash + Sized {
    fn x(self) -> i32;
    fn y(self) -> i32;
    fn from_xy(x: i32, y: i32) -> Self;

    fn offset(self, dx: i32, dy: i32) -> Self {
        Self::from_xy(self.x() + dx, self.y() + dy)
    }

    // Move one step towards direction
    fn step(self, dir: Direction) -> Self {
        let (dx, dy) = dir.delta();
        self.offset(dx, dy)
    }

    // 4 cardinal neighbors (North, East, South, West)
    fn neighbors4(self) -> [Self; 4] {
        Direction::CARDINAL.map(|d| self.step(d))
    }
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
