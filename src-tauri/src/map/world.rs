use crate::map::{MapTile, RegionId, TileCoord, TileRect, error::MapError, region::RegionRegistry};

#[derive(Debug, Clone)]
pub struct WorldMap {
    bounds: TileRect,
    tiles: Vec<MapTile>,
    regions: RegionRegistry,
}

impl WorldMap {
    pub fn new(bounds: TileRect) -> Self {
        let count = bounds.width() as usize * bounds.height() as usize;
        Self {
            bounds,
            tiles: vec![MapTile::default(); count],
            regions: RegionRegistry::new(),
        }
    }

    pub fn bounds(&self) -> TileRect {
        self.bounds
    }

    pub fn regions(&self) -> &RegionRegistry {
        &self.regions
    }

    pub fn regions_mut(&mut self) -> &mut RegionRegistry {
        &mut self.regions
    }

    pub fn contains(&self, coord: TileCoord) -> bool {
        self.bounds.contains(coord)
    }

    pub fn tile(&self, coord: TileCoord) -> Result<&MapTile, MapError> {
        let i = self.index(coord)?;
        Ok(&self.tiles[i])
    }

    pub fn tile_mut(&mut self, coord: TileCoord) -> Result<&mut MapTile, MapError> {
        let i = self.index(coord)?;
        Ok(&mut self.tiles[i])
    }

    // Set with check on if region exists
    pub fn assign_region(&mut self, coord: TileCoord, region: RegionId) -> Result<(), MapError> {
        if !self.regions.contains(region) {
            return Err(MapError::UnknownRegion(region));
        }
        self.tile_mut(coord)?.set_region(Some(region));
        Ok(())
    }

    pub fn tiles_in(&self, rect: TileRect) -> impl Iterator<Item = (TileCoord, &MapTile)> {
        self.clip(rect)
            .into_iter()
            .flat_map(|r| r.iter())
            .map(move |c| (c, &self.tiles[self.offset(c)]))
    }

    pub fn tiles_in_region(&self, region: RegionId) -> impl Iterator<Item = TileCoord> {
        self.bounds.iter().filter(move |&c| {
            self.tiles[self.offset(c)]
                .region()
                .is_some_and(|r| self.regions.is_within(r, region))
        })
    }

    fn index(&self, coord: TileCoord) -> Result<usize, MapError> {
        if self.bounds.contains(coord) {
            Ok(self.offset(coord))
        } else {
            Err(MapError::OutOfBounds(coord))
        }
    }

    // Row-major offset, the coord must be inside bounds
    fn offset(&self, coord: TileCoord) -> usize {
        debug_assert!(self.bounds.contains(coord));
        let min = self.bounds.min();
        (coord.y - min.y) as usize * self.bounds.width() as usize + (coord.x - min.x) as usize
    }

    fn clip(&self, rect: TileRect) -> Option<TileRect> {
        if !rect.intersects(&self.bounds) {
            return None;
        }
        let (b_min, b_max) = (self.bounds.min(), self.bounds.max());
        Some(TileRect::new(
            TileCoord::new(rect.min().x.max(b_min.x), rect.min().y.max(b_min.y)),
            TileCoord::new(rect.max().x.min(b_max.x), rect.max().y.min(b_max.y)),
        ))
    }
}
