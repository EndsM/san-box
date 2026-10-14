// Use a enum for terrains for now
pub enum Terrain {
    Plain,
    Grassland,
    Forest,
    Hill,
    Mountain,
    Desert,
    Swamp,
    Lake,
    River,
    Sea,
}

impl Terrain {
    pub const fn is_water(self) -> bool {
        matches!(self, Terrain::Lake | Terrain::Sea | Terrain::River)
    }
}

// Tile could be on a road
pub enum RoadLevel {
    None,
    Path,
    Road,
    Highway,
}

pub struct Tile {}
