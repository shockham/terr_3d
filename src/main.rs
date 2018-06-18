extern crate caper;
extern crate simdnoise;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{DefaultTag, RenderItemBuilder, TransformBuilder, Transform};
use caper::utils::handle_fp_inputs;

use simdnoise::{get_3d_scaled_noise, NoiseType::Fbm};

use std::iter;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();


    const MAP_SIZE:usize = 200;
    const MAP_SIZE_2:usize = MAP_SIZE * MAP_SIZE;
    const MAP_SIZE_3:usize = MAP_SIZE * MAP_SIZE * MAP_SIZE;

    let verts = (0..MAP_SIZE)
        .flat_map(|x| iter::repeat(x).take(MAP_SIZE))
        .zip((0..MAP_SIZE).cycle().take(MAP_SIZE_3))
        .cycle()
        .take(MAP_SIZE_3)
        .zip((0..MAP_SIZE).flat_map(|z| iter::repeat(z).take(MAP_SIZE_2)))
        .map(|((x, y), z)| (x as f32, y as f32, z as f32))
        .collect::<Vec<(f32, f32, f32)>>();

    //println!("verts:{:?}", verts);

    //  Set your noise type
    let noise_type = Fbm {
          freq: 0.04,
          lacunarity: 0.5,
          gain: 2.0,
          octaves: 3,
    };

    // Get a block of 200x200x200 3d noise
    let an_f32_vec = 
        get_3d_scaled_noise(0.0, MAP_SIZE, 0.0, MAP_SIZE,0.0, MAP_SIZE, noise_type, 0.0, 1.0);

    //println!("heights:{:?}", an_f32_vec);

    let transforms = verts.iter()
        .zip(an_f32_vec)
        .filter(|(_, height)| *height > 0.80f32)
        .map(|(&pos, _)| {
            TransformBuilder::default()
                .pos(pos)
                .build()
                .unwrap()
        }).collect::<Vec<Transform>>();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(transforms)
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut g.cams[0]);

                // quit
                if g.input.keys_down.contains(&Key::Escape) {
                    return UpdateStatus::Finish;
                }

                UpdateStatus::Continue
            },
        );

        if let UpdateStatus::Finish = status {
            break;
        }
    }
}
