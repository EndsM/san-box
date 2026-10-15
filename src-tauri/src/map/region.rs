use std::{collections::BTreeMap, iter};

use serde::{Deserialize, Serialize};

use crate::map::{RegionId, error::MapError};

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

    // Add a region, `parent: None` makes it a top layer region
    pub fn add(
        &mut self,
        name: impl Into<String>,
        parent: Option<RegionId>,
    ) -> Result<RegionId, MapError> {
        if let Some(p) = parent
            && !self.contains(p)
        {
            return Err(MapError::UnknownRegion(p));
        }
        let id = RegionId::new(self.next_id);
        self.next_id += 1;
        self.regions.insert(
            id,
            Region {
                id,
                name: name.into(),
                parent,
            },
        );
        Ok(id)
    }

    pub fn get(&self, id: RegionId) -> Option<&Region> {
        self.regions.get(&id)
    }

    pub fn contains(&self, id: RegionId) -> bool {
        self.regions.contains_key(&id)
    }

    pub fn len(&self) -> usize {
        self.regions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.values()
    }

    // Top layer regions
    pub fn roots(&self) -> impl Iterator<Item = &Region> {
        self.iter().filter(|r| r.parent.is_none())
    }

    // Direct children only
    pub fn children(&self, id: RegionId) -> impl Iterator<Item = &Region> {
        self.iter().filter(move |r| r.parent == Some(id))
    }

    pub fn ancestors(&self, id: RegionId) -> impl Iterator<Item = RegionId> {
        let parent_of = |r: RegionId| self.get(r).and_then(|region| region.parent);
        iter::successors(parent_of(id), move |&p| parent_of(p))
    }

    // True if `id` is `ancestor` itself or sits anywhere below it
    pub fn is_within(&self, id: RegionId, ancestor: RegionId) -> bool {
        id == ancestor || self.ancestors(id).any(|a| a == ancestor)
    }
}
