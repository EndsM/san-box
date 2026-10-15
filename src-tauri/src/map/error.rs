use std::fmt::{Display, Formatter, Result};

use crate::map::{RegionId, TileCoord, id::SettlementId};

// List of error types could occur for map system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapError {
    OutOfBounds(TileCoord),
    UnknownRegion(RegionId),
    UnknownSettlement(SettlementId),
    EmptyFootprint,
    DisconnectedFootprint, // As for now, Settlement footprint should be connected
}

impl Display for MapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MapError::OutOfBounds(c) => write!(f, "tile ({}, {}) is out of map bounds", c.x, c.y),
            MapError::UnknownRegion(id) => write!(f, "unknown region {id}"),
            MapError::UnknownSettlement(id) => write!(f, "unknown settlement {id}"),
            MapError::EmptyFootprint => write!(f, "settlement footprint is empty"),
            MapError::DisconnectedFootprint => {
                write!(f, "settlement footprint is not edge-connected")
            }
        }
    }
}
