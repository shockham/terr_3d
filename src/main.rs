extern crate caper;
extern crate simdnoise;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;

mod movement;
mod setup;
mod state;
mod terrain;
mod update;
mod shaders;

use movement::HandleInput;
use setup::Setup;
use update::ItemUpdate;

#[derive(Clone)]
pub enum Tags {
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
    let (mut game, event_loop) = Game::<Tags>::new();
    let mut state = game.setup();

    // run the engine update
    start_loop(event_loop, move |events| {
        game.update(
            |_: &Ui| {},
            |g: &mut Game<Tags>| -> UpdateStatus {
                // update the first person inputs
                g.handle_inputs(&mut state.pseu_cam);

                if state.alive == true {
                    g.item_update(&mut state);
                }

                // TODO refactor this so something more appropriate
                if state.alive == false && g.input.keys_down.contains(&Key::R) {
                    state.alive = true;
                    // reset objects
                    g.get_render_item(0).active = true;
                    g.get_text_item(0).text = "".to_string();
                    g.get_text_item(2).active = false;
                    state.pseu_cam.pos = (0f32, 0f32, 0f32);
                }

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
            events,
        )
    });
}
