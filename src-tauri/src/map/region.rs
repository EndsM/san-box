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
