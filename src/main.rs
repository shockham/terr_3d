extern crate caper;
extern crate simdnoise;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{CameraBuilder, DefaultTag, RenderItemBuilder};
use caper::utils::handle_fp_inputs;

mod terrain;

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<DefaultTag>::new();

    game.cams[0].pos = (50f32, 50f32, 50f32);

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
