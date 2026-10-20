import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css";
import MapCanvas from "./map/map-canvas";
import type { MapSnapshot } from "./map/types";

function App() {
  const [map, setMap] = useState<MapSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    invoke<MapSnapshot>("get_world_snapshot")
      .then(setMap)
      .catch((e) => setError(String(e)));
  }, []);

  if (error) return <div className="status">Failed to load map: {error}</div>;
  if (!map) return <div className="status">Loading map…</div>;
  return <MapCanvas map={map} />;
}

export default App;
