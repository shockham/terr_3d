extern crate caper;
extern crate simdnoise;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;
use caper::mesh::gen_cube;
use caper::types::{CameraBuilder, RenderItemBuilder, MaterialBuilder};

use rayon::prelude::*;

mod movement;
mod terrain;

use terrain::{HALF_MAP_SIZE, MAP_SIZE, SCALE};

#[derive(Clone)]
enum Tags {
    NoOp,
    Terrain,
}

impl Default for Tags {
    fn default() -> Tags {
        Tags::NoOp
    }
}

fn main() {
    // crate an instance of the game struct
    let mut game = Game::<Tags>::new();

    game.cams[0].pos = (HALF_MAP_SIZE, HALF_MAP_SIZE, (MAP_SIZE as f32 * SCALE) - 70f32);

    let mut pseu_cam = CameraBuilder::default().build().unwrap();

    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .name("terrain".into())
            .vertices(gen_cube())
            .instance_transforms(terrain::get_transforms(pseu_cam.pos))
            .tag(Tags::Terrain)
            .material(MaterialBuilder::default().shader_name("dist".into()).build().unwrap())
            .build()
            .unwrap(),
    );

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<Tags>| -> UpdateStatus {
                // update the first person inputs
                if g.input.hide_mouse {
                    movement::handle_inputs(&mut g.input, &mut pseu_cam, g.delta);
                    g.cams[0].euler_rot = pseu_cam.euler_rot;
                }

                pseu_cam.pos.2 -= 20f32 * g.delta;

                // deal with the diff render item types
                g.render_items_iter_mut().for_each(|ri| match ri.tag {
                    Tags::Terrain => {
                        ri.instance_transforms = terrain::get_transforms(pseu_cam.pos);
                    }
                    Tags::NoOp => (),
                });

                // editor stuff
                if g.input.keys_down.contains(&Key::LShift) {
                    if g.input.keys_down.contains(&Key::L) {
                        g.renderer.show_editor = true;
                    }
                    if g.input.keys_down.contains(&Key::K) {
                        g.renderer.show_editor = false;
                    }
                    g.input.hide_mouse = !g.input.keys_down.contains(&Key::M);
                }
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
