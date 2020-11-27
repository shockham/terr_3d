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
        // deal with the diff render item types
        self.render_items_iter_mut().for_each(|ri| match ri.tag {
            Tags::Terrain if ri.active == true => {
                ri.instance_transforms = terrain::get_transforms(state.pseu_cam.pos);
                if ri.instance_transforms.par_iter().any(|&t| t.pos == (75f32, 75f32, 90f32)) {
                    ri.active = false;
                }
            }
            Tags::Terrain => (),
            Tags::NoOp => (),
        });

        // TODO: fix this being set by whether the terrain is active
        state.alive = self.get_render_item(0).active;

        if state.alive == false {
            self.get_text_item(0).text = if state.pseu_cam.pos.2 < 0f32 {
                "YOU DIED".to_string()
            } else {
                "".to_string()
            };
        } else {
            self.get_text_item(1).text = format!("score: {:.0}", state.pseu_cam.pos.2.abs());
            // continually move forward
            state.pseu_cam.pos.2 -= 20f32 * self.delta;
        }
    }
}
