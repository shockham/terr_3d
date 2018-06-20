extern crate caper;
extern crate simdnoise;
#[macro_use]
extern crate lazy_static;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{CameraBuilder, DefaultTag, RenderItemBuilder};
use caper::utils::handle_fp_inputs;

mod terrain;

use terrain::HALF_MAP_SIZE;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    game.cams[0].pos = (HALF_MAP_SIZE, HALF_MAP_SIZE, HALF_MAP_SIZE);

    let mut pseu_cam = CameraBuilder::default().build().unwrap();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .vertices(gen_cube())
            .instance_transforms(terrain::get_transforms(pseu_cam.pos))
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<DefaultTag>| -> UpdateStatus {
                // update the first person inputs
                handle_fp_inputs(&mut g.input, &mut pseu_cam);
                g.cams[0].euler_rot = pseu_cam.euler_rot;

                g.get_render_item(0).instance_transforms = terrain::get_transforms(pseu_cam.pos);

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
