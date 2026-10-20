use serde::Serialize;

use crate::map::{
    MapTile, Region, RegionId, RoadLevel, Settlement, SettlementId, SettlementType, Terrain,
    TileCoord, WorldMap,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TileView {
    pub terrain: Terrain,
    pub elevation: u8,
    pub road: RoadLevel,
    pub region: Option<RegionId>,
    pub settlement: Option<SettlementId>,
}

impl From<&MapTile> for TileView {
    fn from(t: &MapTile) -> Self {
        Self {
            terrain: t.terrain(),
            elevation: t.elevation(),
            road: t.road(),
            region: t.region(),
            settlement: t.settlement(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionView {
    pub id: RegionId,
    pub name: String,
    pub parent: Option<RegionId>,
}

impl From<&Region> for RegionView {
    fn from(r: &Region) -> Self {
        Self {
            id: r.id(),
            name: r.name().to_owned(),
            parent: r.parent(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementView {
    pub id: SettlementId,
    pub name: String,
    pub settlement_type: SettlementType,
    pub footprint: Vec<TileCoord>,
}

impl From<&Settlement> for SettlementView {
    fn from(s: &Settlement) -> Self {
        Self {
            id: s.id(),
            name: s.name().to_owned(),
            settlement_type: s.settlement_type(),
            footprint: s.footprint().iter().copied().collect(),
        }
    }
}

// `tiles` is row-major: index = (y - min_y) * width + (x - min_x)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapSnapshot {
    pub min_x: i32,
    pub min_y: i32,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileView>,
    pub regions: Vec<RegionView>,
    pub settlements: Vec<SettlementView>,
}

impl MapSnapshot {
    pub fn from_world(world: &WorldMap) -> Self {
        let b = world.bounds();
        Self {
            min_x: b.min().x,
            min_y: b.min().y,
            width: b.width(),
            height: b.height(),
            tiles: world.tiles_in(b).map(|(_, t)| TileView::from(t)).collect(),
            regions: world.regions().iter().map(RegionView::from).collect(),
            settlements: world
                .settlements()
                .iter()
                .map(SettlementView::from)
                .collect(),
        }
    }
}
