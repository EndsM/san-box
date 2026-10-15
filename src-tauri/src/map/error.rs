use std::fmt::{Display, Formatter, Result};

use crate::map::{RegionId, TileCoord};

// List of error types could occur for map system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapError {
    OutOfBounds(TileCoord),
    UnknownRegion(RegionId),
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MapError::OutOfBounds(c) => write!(f, "tile ({}, {}) is out of map bounds", c.x, c.y),
            MapError::UnknownRegion(id) => write!(f, "unknown region {id}"),
        }
    }
}
