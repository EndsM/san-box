use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::map::{GridCoord, MapError, TileCoord, TileRect, id::SettlementId};

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

pub(super) fn validate_shape(tiles: &BTreeSet<TileCoord>) -> Result<(), MapError> {
    let Some(&start) = tiles.iter().next() else {
        return Err(MapError::EmptyFootprint);
    };
    let mut seen = BTreeSet::from([start]);
    let mut stack = vec![start];
    while let Some(c) = stack.pop() {
        for n in c.neighbors4() {
            if tiles.contains(&n) && seen.insert(n) {
                stack.push(n);
            }
        }
    }
    if seen.len() == tiles.len() {
        Ok(())
    } else {
        Err(MapError::DisconnectedFootprint)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SettlementRegistry {
    // BTreeMap keeps iteration order stable, which keeps saves and tests deterministic
    settlements: BTreeMap<SettlementId, Settlement>,
    next_id: u32,
}

impl SettlementRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    // The footprint must already be validated
    pub(super) fn insert(
        &mut self,
        name: impl Into<String>,
        settlement_type: SettlementType,
        footprint: BTreeSet<TileCoord>,
    ) -> SettlementId {
        let id = SettlementId::new(self.next_id);
        self.next_id += 1;
        self.settlements.insert(
            id,
            Settlement {
                id,
                name: name.into(),
                settlement_type,
                footprint,
            },
        );
        id
    }

    pub fn get(&self, id: SettlementId) -> Option<&Settlement> {
        self.settlements.get(&id)
    }

    pub fn contains(&self, id: SettlementId) -> bool {
        self.settlements.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.settlements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.settlements.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Settlement> {
        self.settlements.values()
    }
}
