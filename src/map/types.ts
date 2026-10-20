export type Terrain =
  | "Plain" | "Grassland" | "Forest" | "Hill" | "Mountain"
  | "Desert" | "Swamp" | "Lake" | "River" | "Sea";

export type RoadLevel = "None" | "Path" | "Road" | "Highway";

export interface TileView {
  terrain: Terrain;
  elevation: number;
  road: RoadLevel;
  region: number | null;
  settlement: number | null;
}

export interface RegionView {
  id: number;
  name: string;
  parent: number | null;
}

export interface SettlementView {
  id: number;
  name: string;
  settlementType: "City" | "Village";
  footprint: { x: number; y: number }[];
}

export interface MapSnapshot {
  minX: number;
  minY: number;
  width: number;
  height: number;
  tiles: TileView[]; // row-major
  regions: RegionView[];
  settlements: SettlementView[];
}
