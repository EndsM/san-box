use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::map::RegionId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Region {
    id: RegionId,
    name: String,
    parent: Option<RegionId>,
}

impl Region {
    pub fn id(&self) -> RegionId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parent(&self) -> Option<RegionId> {
        self.parent
    }
}

// Used for holding regions, instead of holding at WorldMap
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegionRegistry {
    // BTreeMap keeps iteration order stable, which keeps saves and tests deterministic
    regions: BTreeMap<RegionId, Region>,
    next_id: u32,
}

impl RegionRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}
