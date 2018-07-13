extern crate caper;
extern crate simdnoise;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

use caper::game::*;
use caper::imgui::Ui;
use caper::input::Key;

mod movement;
mod terrain;
mod setup;
mod state;
mod update;

use setup::Setup;
use movement::HandleInput;
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
    let mut game = Game::<Tags>::new();
    let mut state = game.setup();

    loop {
        // run the engine update
        let status = game.update(
            |_: &Ui| {},
            |g: &mut Game<Tags>| -> UpdateStatus {
                // update the first person inputs
                g.handle_inputs(&mut state.pseu_cam);

                g.item_update(&mut state);

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
