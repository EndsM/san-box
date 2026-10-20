use std::collections::BTreeSet;

use crate::map::{MapTile, SettlementType, Terrain, TileCoord, TileRect, WorldMap};

pub const DEMO_W: i32 = 64;
pub const DEMO_H: i32 = 48;

fn hash2(x: i32, y: i32) -> u32 {
    let mut h = (x as u32).wrapping_mul(374_761_393) ^ (y as u32).wrapping_mul(668_265_263);
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

fn terrain_at(x: i32, y: i32) -> Terrain {
    if x >= 56 {
        return Terrain::Sea;
    }
    if x == 30 + (y / 8) % 3 {
        return Terrain::River;
    }
    if (40..43).contains(&x) && (30..33).contains(&y) {
        return Terrain::Lake;
    }
    // Blocky 3x3 clusters so terrain forms patches instead of pure noise
    match hash2(x / 3, y / 3) % 10 {
        0..=2 => Terrain::Plain,
        3..=4 => Terrain::Grassland,
        5..=6 => Terrain::Forest,
        7 => Terrain::Hill,
        8 => Terrain::Mountain,
        _ => {
            if x < 10 {
                Terrain::Desert
            } else {
                Terrain::Swamp
            }
        }
    }
}

pub fn demo_world() -> WorldMap {
    let bounds = TileRect::new(TileCoord::new(0, 0), TileCoord::new(DEMO_W - 1, DEMO_H - 1));
    let mut world = WorldMap::new(bounds);

    for c in bounds.iter() {
        *world.tile_mut(c).expect("in bounds") = MapTile::new(terrain_at(c.x, c.y));
    }

    // Provinces (州) with two commanderies (郡) each
    let regs = world.regions_mut();
    let yi = regs.add("益州", None).expect("root");
    let yi_n = regs.add("汉中郡", Some(yi)).expect("parent exists");
    let yi_s = regs.add("蜀郡", Some(yi)).expect("parent exists");
    let jing = regs.add("荆州", None).expect("root");
    let jing_n = regs.add("南阳郡", Some(jing)).expect("parent exists");
    let jing_s = regs.add("长沙郡", Some(jing)).expect("parent exists");
    let yang = regs.add("扬州", None).expect("root");
    let yang_n = regs.add("丹阳郡", Some(yang)).expect("parent exists");
    let yang_s = regs.add("会稽郡", Some(yang)).expect("parent exists");

    for c in bounds.iter() {
        if world.tile(c).expect("in bounds").terrain() == Terrain::Sea {
            continue;
        }
        let (north, south) = match c.x {
            ..22 => (yi_n, yi_s),
            22..42 => (jing_n, jing_s),
            _ => (yang_n, yang_s),
        };
        let region = if c.y < 24 { north } else { south };
        world.assign_region(c, region).expect("region exists");
    }

    let shape = |cells: &[(i32, i32)]| -> BTreeSet<TileCoord> {
        cells.iter().map(|&(x, y)| TileCoord::new(x, y)).collect()
    };
    let add = |w: &mut WorldMap, name: &str, ty: SettlementType, cells: &[(i32, i32)]| {
        w.add_settlement(name, ty, shape(cells))
            .expect("demo settlement is valid");
    };

    add(
        &mut world,
        "成都",
        SettlementType::City,
        &[(8, 30), (9, 30), (8, 31), (9, 31)],
    );
    add(&mut world, "汉中", SettlementType::Village, &[(14, 10)]);
    add(
        &mut world,
        "襄阳",
        SettlementType::City,
        &[(36, 14), (37, 14), (37, 15)],
    );
    add(
        &mut world,
        "建业",
        SettlementType::City,
        &[(48, 20), (49, 20), (48, 21), (49, 21)],
    );
    add(&mut world, "长沙", SettlementType::Village, &[(38, 36)]);

    world
}
