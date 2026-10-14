use std::hash::Hash;

use serde::{Deserialize, Serialize};

// The sub tile is within a world tile
pub const SUB_TILE_SIZE: usize = 15;
// Calculate the result count here for now
pub const SUB_TILE_COUNT: usize = SUB_TILE_SIZE * SUB_TILE_SIZE;

pub const SIZE_I32: i32 = SUB_TILE_SIZE as i32;

// Directions may used later for unit facing, pathfinding, etc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    // 8 neighbors (clockwise from North)
    fn neighbors8(self) -> [Self; 8] {
        Direction::ALL.map(|d| self.step(d))
    }

    // Manhattan distance (4-way steps)
    fn manhattan_distance(self, other: Self) -> u32 {
        self.x()
            .abs_diff(other.x())
            .saturating_add(self.y().abs_diff(other.y()))
    }

    // Chebyshev distance (8-way steps)
    fn chebyshev_distance(self, other: Self) -> u32 {
        self.x()
            .abs_diff(other.x())
            .max(self.y().abs_diff(other.y()))
    }
}

// This represent a tile's coord on the big map
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct TileCoord {
    pub x: i32,
    pub y: i32,
}

impl TileCoord {
    pub const ORIGIN: TileCoord = TileCoord { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Translate to top-left subtile's Global Coord
    pub const fn sub_origin(self) -> SubTileCoord {
        SubTileCoord::new(self.x * SIZE_I32, self.y * SIZE_I32)
    }

    // Translate a local sub-tile to its global coordinate
    pub const fn sub(self, local: LocalCoord) -> SubTileCoord {
        SubTileCoord::new(
            self.x * SIZE_I32 + local.x as i32,
            self.y * SIZE_I32 + local.y as i32,
        )
    }
}

impl GridCoord for TileCoord {
    fn x(self) -> i32 {
        self.x
    }

    fn y(self) -> i32 {
        self.y
    }

    fn from_xy(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for TileCoord {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

// This represent a subtile's coord in its Tile, thus LocalCoord
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
#[serde(try_from = "LocalCoordRepr")]
pub struct LocalCoord {
    x: u8,
    y: u8,
}

// Shadow struct for serd and deserd check
#[derive(Deserialize)]
struct LocalCoordRepr {
    x: u8,
    y: u8,
}

impl TryFrom<LocalCoordRepr> for LocalCoord {
    type Error = String;

    fn try_from(r: LocalCoordRepr) -> Result<Self, Self::Error> {
        LocalCoord::new(r.x, r.y).ok_or_else(|| {
            format!(
                "local coord ({}, {}) out of range 0..{}",
                r.x, r.y, SUB_TILE_SIZE
            )
        })
    }
}

impl LocalCoord {
    pub const CENTER: LocalCoord = LocalCoord {
        x: (SUB_TILE_SIZE / 2) as u8,
        y: (SUB_TILE_SIZE / 2) as u8,
    };

    // Validates range, returns None if out of bounds
    pub const fn new(x: u8, y: u8) -> Option<Self> {
        if (x as usize) < SUB_TILE_SIZE && (y as usize) < SUB_TILE_SIZE {
            Some(Self { x, y })
        } else {
            None
        }
    }

    pub const fn x(self) -> u8 {
        self.x
    }

    pub const fn y(self) -> u8 {
        self.y
    }

    // Row-major linear index for compact flat-array storage [T; SUB_TILE_COUNT]
    pub const fn index(self) -> usize {
        self.y as usize * SUB_TILE_SIZE + self.x as usize
    }

    // Inverse of index(), returns None if out of range
    pub const fn from_index(index: usize) -> Option<Self> {
        if index < SUB_TILE_COUNT {
            Some(Self {
                x: (index % SUB_TILE_SIZE) as u8,
                y: (index / SUB_TILE_SIZE) as u8,
            })
        } else {
            None
        }
    }

    // Check if sub-tile at edge
    pub const fn is_edge(self) -> bool {
        let max = (SUB_TILE_SIZE - 1) as u8;
        self.x == 0 || self.y == 0 || self.x == max || self.y == max
    }

    // Row-major iterator over all local coordinates
    pub fn all() -> impl Iterator<Item = LocalCoord> {
        (0..SUB_TILE_COUNT).map(|i| LocalCoord::from_index(i).expect("index in range"))
    }
}

// This represent a subtile's global Coord, which means a subtile now can be traced globally.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
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

    pub const fn local(self) -> LocalCoord {
        LocalCoord {
            x: self.x.rem_euclid(SIZE_I32) as u8,
            y: self.y.rem_euclid(SIZE_I32) as u8,
        }
    }

    pub const fn split(self) -> (TileCoord, LocalCoord) {
        (self.tile(), self.local())
    }

    pub const fn from_parts(tile: TileCoord, local: LocalCoord) -> Self {
        tile.sub(local)
    }
}

impl GridCoord for SubTileCoord {
    fn x(self) -> i32 {
        self.x
    }
    fn y(self) -> i32 {
        self.y
    }
    fn from_xy(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for SubTileCoord {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

// Represent subtile's local coord in a battle map
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct BattleCoord {
    x: i32,
    y: i32,
}

impl BattleCoord {
    pub const ORIGIN: BattleCoord = BattleCoord { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl GridCoord for BattleCoord {
    fn x(self) -> i32 {
        self.x
    }
    fn y(self) -> i32 {
        self.y
    }
    fn from_xy(x: i32, y: i32) -> Self {
        Self::new(x, y)
    }
}

impl From<(i32, i32)> for BattleCoord {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

// A rectangle of tiles, can represent a map area, or a viewport, etc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileRect {
    min: TileCoord,
    max: TileCoord,
}

impl TileRect {
    // Auto construct from two diagonal corners
    pub fn new(a: TileCoord, b: TileCoord) -> Self {
        Self {
            min: TileCoord::new(a.x.min(b.x), a.y.min(b.y)),
            max: TileCoord::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }

    // In case a single tile rect is needed
    pub fn single(tile: TileCoord) -> Self {
        Self {
            min: tile,
            max: tile,
        }
    }

    // Minimum bounding rect containing all given tiles; None if empty
    pub fn bounding<I: IntoIterator<Item = TileCoord>>(tiles: I) -> Option<Self> {
        let mut iter = tiles.into_iter();
        let first = iter.next()?;
        Some(iter.fold(Self::single(first), |r, t| r.including(t)))
    }

    // Expand rect to include tile
    pub fn including(self, tile: TileCoord) -> Self {
        Self {
            min: TileCoord::new(self.min.x.min(tile.x), self.min.y.min(tile.y)),
            max: TileCoord::new(self.max.x.max(tile.x), self.max.y.max(tile.y)),
        }
    }

    // Expand towards all directions by margin
    pub fn expanded(self, margin: i32) -> Self {
        Self {
            min: self.min.offset(-margin, -margin),
            max: self.max.offset(margin, margin),
        }
    }

    pub fn min(&self) -> TileCoord {
        self.min
    }

    pub fn max(&self) -> TileCoord {
        self.max
    }

    // Width in tiles
    pub fn width(&self) -> u32 {
        self.max.x.abs_diff(self.min.x) + 1
    }

    // Height in tiles
    pub fn height(&self) -> u32 {
        self.max.y.abs_diff(self.min.y) + 1
    }

    pub fn contains(&self, tile: TileCoord) -> bool {
        (self.min.x..=self.max.x).contains(&tile.x) && (self.min.y..=self.max.y).contains(&tile.y)
    }

    pub fn intersects(&self, other: &TileRect) -> bool {
        self.min.x <= other.max.x
            && other.min.x <= self.max.x
            && self.min.y <= other.max.y
            && other.min.y <= self.max.y
    }

    // Row-major iterator over all tiles inside the rect
    pub fn iter(self) -> impl Iterator<Item = TileCoord> {
        (self.min.y..=self.max.y)
            .flat_map(move |y| (self.min.x..=self.max.x).map(move |x| TileCoord::new(x, y)))
    }

    // Width in sub-tiles
    pub fn sub_width(&self) -> usize {
        self.width() as usize * SUB_TILE_SIZE
    }

    // Height in sub-tiles
    pub fn sub_height(&self) -> usize {
        self.height() as usize * SUB_TILE_SIZE
    }

    // Sub-map's top-left corner in global sub-tile coordinates
    pub fn sub_origin(&self) -> SubTileCoord {
        self.min.sub_origin()
    }

    // Convert global sub-tile coord to battlefield local coord; returns None if outside
    pub fn sub_to_battle(&self, sub: SubTileCoord) -> Option<BattleCoord> {
        if !self.contains(sub.tile()) {
            return None;
        }
        let origin = self.sub_origin();
        Some(BattleCoord::new(sub.x - origin.x, sub.y - origin.y))
    }

    // Convert battlefield local coord to global sub-tile coord; returns None if out of bounds
    pub fn battle_to_sub(&self, battle: BattleCoord) -> Option<SubTileCoord> {
        if battle.x < 0
            || battle.y < 0
            || battle.x as usize >= self.sub_width()
            || battle.y as usize >= self.sub_height()
        {
            return None;
        }
        let origin = self.sub_origin();
        Some(SubTileCoord::new(origin.x + battle.x, origin.y + battle.y))
    }
}
