use caper::types::{Transform, TransformBuilder};
use simdnoise::*;
use std::iter;
use rayon::prelude::*;

pub const SCALE: f32 = 2.5f32;
pub const MAP_SIZE: usize = 60;
const MAP_SIZE_2: usize = MAP_SIZE * MAP_SIZE;
const MAP_SIZE_3: usize = MAP_SIZE * MAP_SIZE * MAP_SIZE;

pub const HALF_MAP_SIZE: f32 = (MAP_SIZE / 2) as f32 * SCALE;

lazy_static! {
    static ref VERTS: Vec<(f32, f32, f32)> = {
        (0..MAP_SIZE)
            .flat_map(|x| iter::repeat(x).take(MAP_SIZE))
            .zip((0..MAP_SIZE).cycle().take(MAP_SIZE_3))
            .cycle()
            .take(MAP_SIZE_3)
            .zip((0..MAP_SIZE).flat_map(|z| iter::repeat(z).take(MAP_SIZE_2)))
            .map(|((x, y), z)| (x as f32, y as f32, z as f32))
            .collect::<Vec<(f32, f32, f32)>>()
    };
}

pub fn get_transforms(pos: (f32, f32, f32)) -> Vec<Transform> {
    let an_f32_vec = NoiseBuilder::fbm_3d_offset(
            pos.1, MAP_SIZE, pos.0, MAP_SIZE, pos.2, MAP_SIZE
        )
        .with_freq(0.03)
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(1)
        .generate_scaled(0f32, 1f32);

    VERTS
        .par_iter()
        .zip(an_f32_vec)
        .filter(|(_, height)| *height > 0.7f32 && *height < 0.82f32)
        .map(|(&pos, _)| {
            TransformBuilder::default()
                .pos((pos.0 * SCALE, pos.1 * SCALE, pos.2 * SCALE))
                .scale((SCALE, SCALE, SCALE))
                .build()
                .unwrap()
        })
        .collect::<Vec<Transform>>()
}
