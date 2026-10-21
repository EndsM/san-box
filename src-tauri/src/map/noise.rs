// Deterministic 2D spatial coordinate hash for procedural generation
pub fn hash(seed: u64, x: i32, y: i32) -> u64 {
    let mut h = seed
        ^ (x as u32 as u64).wrapping_mul(0x9E37_79B1_85EB_CA87)
        ^ (y as u32 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    h ^= h >> 33;
    h = h.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    h ^ (h >> 33)
}

// Uniform value in [0, 1) for a lattice point
pub fn unit(seed: u64, x: i32, y: i32) -> f32 {
    (hash(seed, x, y) >> 40) as f32 / 16_777_216.0
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

// Smooth value noise in [0, 1), one lattice cell per unit
pub fn value_noise(seed: u64, x: f32, y: f32) -> f32 {
    let (x0, y0) = (x.floor(), y.floor());
    let (tx, ty) = (smooth(x - x0), smooth(y - y0));
    let (ix, iy) = (x0 as i32, y0 as i32);
    let top = lerp(unit(seed, ix, iy), unit(seed, ix + 1, iy), tx);
    let bottom = lerp(unit(seed, ix, iy + 1), unit(seed, ix + 1, iy + 1), tx);
    lerp(top, bottom, ty)
}

// Fractal noise in [0, 1): `octaves` layers of value noise, each twice as fine and half as strong
pub fn fbm(seed: u64, x: f32, y: f32, octaves: u32) -> f32 {
    let (mut sum, mut amp, mut norm, mut freq) = (0.0, 1.0, 0.0, 1.0);
    for i in 0..octaves.max(1) {
        let layer_seed = seed.wrapping_add(u64::from(i).wrapping_mul(0x9E37_79B9));
        sum += amp * value_noise(layer_seed, x * freq, y * freq);
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}
