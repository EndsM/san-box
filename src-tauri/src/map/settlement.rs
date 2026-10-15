use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::map::{TileCoord, TileRect, id::SettlementId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SettlementType {
    City,
    Village,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settlement {
    id: SettlementId,
    name: String,
    settlement_type: SettlementType,
    footprint: BTreeSet<TileCoord>,
}

impl Settlement {
    pub fn id(&self) -> SettlementId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn settlement_type(&self) -> SettlementType {
        self.settlement_type
    }

    pub fn footprint(&self) -> &BTreeSet<TileCoord> {
        &self.footprint
    }

    pub fn contains(&self, tile: TileCoord) -> bool {
        self.footprint.contains(&tile)
    }

    // Smallest rect covering the area it took, handy as a viewport or a battle area
    pub fn bounds(&self) -> Option<TileRect> {
        TileRect::bounding(self.footprint.iter().copied())
    }
}
