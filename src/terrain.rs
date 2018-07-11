use caper::types::{Transform, TransformBuilder};
use simdnoise::{get_3d_scaled_noise, NoiseType::Fbm};
use std::iter;

pub const SCALE: f32 = 2.5f32;
pub const MAP_SIZE: usize = 70;
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
    let noise_type = Fbm {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 1,
    };

    let an_f32_vec = get_3d_scaled_noise(
        pos.1, MAP_SIZE, pos.0, MAP_SIZE, pos.2, MAP_SIZE, noise_type, 0.0, 1.0,
    );

    VERTS
        .iter()
        .zip(an_f32_vec)
        .filter(|(_, height)| *height > 0.70f32)
        .map(|(&pos, _)| {
            TransformBuilder::default()
                .pos((pos.0 * SCALE, pos.1 * SCALE, pos.2 * SCALE))
                .scale((SCALE, SCALE, SCALE))
                .build()
                .unwrap()
        })
        .collect::<Vec<Transform>>()
}
