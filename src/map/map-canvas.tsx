import { useEffect, useMemo, useRef, useState } from "react";
import type { MapSnapshot, Terrain } from "./types";

const TERRAIN_COLOR: Record<Terrain, string> = {
  Plain: "#c8d68f",
  Grassland: "#8fbf6a",
  Forest: "#4f8a4b",
  Hill: "#a89f6a",
  Mountain: "#7d7770",
  Desert: "#e3cf8e",
  Swamp: "#6f8f7a",
  Lake: "#5fa4d8",
  River: "#4a90d9",
  Sea: "#2f5f9e",
};

const MIN_SCALE = 4;
const MAX_SCALE = 64;

interface View {
  scale: number; // screen px per tile
  x: number; // screen px of the map's top-left corner
  y: number;
}

export default function MapCanvas({ map }: { map: MapSnapshot }) {
  const wrapRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fitted = useRef(false);
  const drag = useRef<{ sx: number; sy: number; vx: number; vy: number } | null>(null);

  const [size, setSize] = useState({ w: 0, h: 0 });
  const [view, setView] = useState<View>({ scale: 10, x: 0, y: 0 });
  const [hover, setHover] = useState<{ x: number; y: number } | null>(null);
  const [showRegions, setShowRegions] = useState(true);
  const [showGrid, setShowGrid] = useState(true);

  // Lookup tables derived from the snapshot
  const lookup = useMemo(() => {
    const regionById = new Map(map.regions.map((r) => [r.id, r]));
    const rootOf = (id: number): number => {
      let cur = regionById.get(id);
      while (cur && cur.parent !== null) cur = regionById.get(cur.parent);
      return cur ? cur.id : id;
    };
    const color = new Map<number, string>();
    for (const r of map.regions) {
      const root = rootOf(r.id);
      const hue = (root * 137.5) % 360;
      const light = root === r.id ? 50 : 38 + ((r.id * 7) % 22);
      color.set(r.id, `hsl(${hue} 65% ${light}%)`);
    }
    const rootGrid = new Int32Array(map.tiles.length);
    map.tiles.forEach((t, i) => (rootGrid[i] = t.region === null ? -1 : rootOf(t.region)));
    const settlementById = new Map(map.settlements.map((s) => [s.id, s]));
    const regionPath = (id: number) => {
      const names: string[] = [];
      let cur = regionById.get(id);
      while (cur) {
        names.unshift(cur.name);
        cur = cur.parent === null ? undefined : regionById.get(cur.parent);
      }
      return names.join(" › ");
    };
    return { color, rootGrid, settlementById, regionPath };
  }, [map]);

  // Track container size
  useEffect(() => {
    const el = wrapRef.current;
    if (!el) return;
    const ro = new ResizeObserver(() => setSize({ w: el.clientWidth, h: el.clientHeight }));
    ro.observe(el);
    setSize({ w: el.clientWidth, h: el.clientHeight });
    return () => ro.disconnect();
  }, []);

  // Fit the map to the window once
  useEffect(() => {
    if (fitted.current || size.w === 0) return;
    const scale = Math.min(size.w / map.width, size.h / map.height);
    setView({
      scale,
      x: (size.w - map.width * scale) / 2,
      y: (size.h - map.height * scale) / 2,
    });
    fitted.current = true;
  }, [size, map]);

  // Wheel zoom around the cursor (needs a non-passive listener)
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const onWheel = (e: WheelEvent) => {
      e.preventDefault();
      const rect = canvas.getBoundingClientRect();
      const mx = e.clientX - rect.left;
      const my = e.clientY - rect.top;
      setView((v) => {
        const scale = Math.min(MAX_SCALE, Math.max(MIN_SCALE, v.scale * (e.deltaY < 0 ? 1.15 : 1 / 1.15)));
        const k = scale / v.scale;
        return { scale, x: mx - (mx - v.x) * k, y: my - (my - v.y) * k };
      });
    };
    canvas.addEventListener("wheel", onWheel, { passive: false });
    return () => canvas.removeEventListener("wheel", onWheel);
  }, []);

  // Draw
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || size.w === 0) return;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = size.w * dpr;
    canvas.height = size.h * dpr;
    const ctx = canvas.getContext("2d")!;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.fillStyle = "#1b1b1f";
    ctx.fillRect(0, 0, size.w, size.h);

    const { scale, x: ox, y: oy } = view;
    const x0 = Math.max(0, Math.floor(-ox / scale));
    const y0 = Math.max(0, Math.floor(-oy / scale));
    const x1 = Math.min(map.width, Math.ceil((size.w - ox) / scale));
    const y1 = Math.min(map.height, Math.ceil((size.h - oy) / scale));
    const at = (tx: number, ty: number) =>
      tx < 0 || ty < 0 || tx >= map.width || ty >= map.height ? null : map.tiles[ty * map.width + tx];

    // Terrain, region tint, settlement fill
    for (let ty = y0; ty < y1; ty++) {
      for (let tx = x0; tx < x1; tx++) {
        const t = map.tiles[ty * map.width + tx];
        const px = ox + tx * scale;
        const py = oy + ty * scale;
        ctx.fillStyle = TERRAIN_COLOR[t.terrain];
        ctx.fillRect(px, py, scale + 0.5, scale + 0.5);
        if (showRegions && t.region !== null) {
          ctx.globalAlpha = 0.35;
          ctx.fillStyle = lookup.color.get(t.region) ?? "#fff";
          ctx.fillRect(px, py, scale + 0.5, scale + 0.5);
          ctx.globalAlpha = 1;
        }
        if (t.settlement !== null) {
          ctx.fillStyle = "#e8c15a";
          ctx.fillRect(px + 1, py + 1, scale - 2, scale - 2);
        }
      }
    }

    // Grid
    if (showGrid && scale >= 16) {
      ctx.strokeStyle = "rgba(0,0,0,0.15)";
      ctx.lineWidth = 1;
      ctx.beginPath();
      for (let tx = x0; tx <= x1; tx++) {
        ctx.moveTo(ox + tx * scale, oy + y0 * scale);
        ctx.lineTo(ox + tx * scale, oy + y1 * scale);
      }
      for (let ty = y0; ty <= y1; ty++) {
        ctx.moveTo(ox + x0 * scale, oy + ty * scale);
        ctx.lineTo(ox + x1 * scale, oy + ty * scale);
      }
      ctx.stroke();
    }

    // Borders: province borders (root region change) and settlement outlines
    const regionPath = new Path2D();
    const settlePath = new Path2D();
    for (let ty = y0; ty < y1; ty++) {
      for (let tx = x0; tx < x1; tx++) {
        const i = ty * map.width + tx;
        const px = ox + tx * scale;
        const py = oy + ty * scale;
        const t = map.tiles[i];
        const right = at(tx + 1, ty);
        const down = at(tx, ty + 1);
        if (right && lookup.rootGrid[i] !== lookup.rootGrid[i + 1]) {
          regionPath.moveTo(px + scale, py);
          regionPath.lineTo(px + scale, py + scale);
        }
        if (down && lookup.rootGrid[i] !== lookup.rootGrid[i + map.width]) {
          regionPath.moveTo(px, py + scale);
          regionPath.lineTo(px + scale, py + scale);
        }
        // Settlement outline: draw an edge wherever the neighbour is a different settlement (or none)
        const edge = (n: typeof t | null, x1_: number, y1_: number, x2_: number, y2_: number) => {
          if ((n?.settlement ?? null) !== t.settlement && (t.settlement !== null || n?.settlement != null)) {
            settlePath.moveTo(x1_, y1_);
            settlePath.lineTo(x2_, y2_);
          }
        };
        edge(right, px + scale, py, px + scale, py + scale);
        edge(down, px, py + scale, px + scale, py + scale);
        if (tx === x0) edge(at(tx - 1, ty), px, py, px, py + scale);
        if (ty === y0) edge(at(tx, ty - 1), px, py, px + scale, py);
      }
    }
    if (showRegions) {
      ctx.strokeStyle = "rgba(20,20,20,0.8)";
      ctx.lineWidth = 2;
      ctx.stroke(regionPath);
    }
    ctx.strokeStyle = "#7a1f1f";
    ctx.lineWidth = 2;
    ctx.stroke(settlePath);

    // Settlement labels
    if (scale >= 8) {
      ctx.font = `${Math.min(18, Math.max(11, scale * 0.9))}px sans-serif`;
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      for (const s of map.settlements) {
        const cx = s.footprint.reduce((a, c) => a + c.x + 0.5, 0) / s.footprint.length;
        const cy = s.footprint.reduce((a, c) => a + c.y + 0.5, 0) / s.footprint.length;
        const px = ox + (cx - map.minX) * scale;
        const py = oy + (cy - map.minY) * scale;
        ctx.lineWidth = 3;
        ctx.strokeStyle = "rgba(255,255,255,0.9)";
        ctx.strokeText(s.name, px, py);
        ctx.fillStyle = "#222";
        ctx.fillText(s.name, px, py);
      }
    }

    // Hover highlight
    if (hover) {
      ctx.strokeStyle = "#fff";
      ctx.lineWidth = 2;
      ctx.strokeRect(ox + hover.x * scale, oy + hover.y * scale, scale, scale);
    }
  }, [map, lookup, view, size, hover, showRegions, showGrid]);

  const tileAt = (e: React.MouseEvent) => {
    const rect = canvasRef.current!.getBoundingClientRect();
    const tx = Math.floor((e.clientX - rect.left - view.x) / view.scale);
    const ty = Math.floor((e.clientY - rect.top - view.y) / view.scale);
    return tx >= 0 && ty >= 0 && tx < map.width && ty < map.height ? { x: tx, y: ty } : null;
  };

  const hovered = hover ? map.tiles[hover.y * map.width + hover.x] : null;

  return (
    <div ref={wrapRef} className="map-wrap">
      <canvas
        ref={canvasRef}
        style={{ width: size.w, height: size.h, cursor: drag.current ? "grabbing" : "grab" }}
        onMouseDown={(e) => (drag.current = { sx: e.clientX, sy: e.clientY, vx: view.x, vy: view.y })}
        onMouseMove={(e) => {
          if (drag.current) {
            const d = drag.current;
            setView((v) => ({ ...v, x: d.vx + e.clientX - d.sx, y: d.vy + e.clientY - d.sy }));
          }
          setHover(tileAt(e));
        }}
        onMouseUp={() => (drag.current = null)}
        onMouseLeave={() => {
          drag.current = null;
          setHover(null);
        }}
      />
      <div className="map-panel">
        <label>
          <input type="checkbox" checked={showRegions} onChange={(e) => setShowRegions(e.target.checked)} /> Regions
        </label>
        <label>
          <input type="checkbox" checked={showGrid} onChange={(e) => setShowGrid(e.target.checked)} /> Grid
        </label>
        <hr />
        {hover && hovered ? (
          <>
            <div>Tile ({hover.x + map.minX}, {hover.y + map.minY})</div>
            <div>Terrain: {hovered.terrain}</div>
            <div>Region: {hovered.region !== null ? lookup.regionPath(hovered.region) : "—"}</div>
            <div>
              Settlement:{" "}
              {hovered.settlement !== null
                ? `${lookup.settlementById.get(hovered.settlement)?.name} (${lookup.settlementById.get(hovered.settlement)?.settlementType})`
                : "—"}
            </div>
          </>
        ) : (
          <div>Hover a tile</div>
        )}
      </div>
    </div>
  );
}
