use super::Tags;
use caper::game::*;
use state::State;
use terrain;

use rayon::prelude::*;

pub trait ItemUpdate {
    fn item_update(&mut self, state: &mut State);
}

impl ItemUpdate for Game<Tags> {
    fn item_update(&mut self, state: &mut State) {
        // continually move forward
        state.pseu_cam.pos.2 -= 20f32 * self.delta;

        // deal with the diff render item types
        self.render_items_iter_mut().for_each(|ri| match ri.tag {
            Tags::Terrain => {
                ri.instance_transforms = terrain::get_transforms(state.pseu_cam.pos);
            }
            Tags::NoOp => (),
        });
    }
}
