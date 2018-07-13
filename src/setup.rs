use caper::types::{RenderItemBuilder, MaterialBuilder};
use caper::game::*;
use terrain::{HALF_MAP_SIZE, MAP_SIZE, SCALE};
use super::Tags;
use caper::mesh::gen_cube;
use terrain;
use state::State;


/// Setup the game
pub fn setup(game: &mut Game<Tags>) -> State {
    // create init state
    let state = State::default();
    // set the cam pos
    game.cams[0].pos = (HALF_MAP_SIZE, HALF_MAP_SIZE, (MAP_SIZE as f32 * SCALE) - 60f32);
    // define some items to be rendered
    game.add_render_item(
        RenderItemBuilder::default()
            .name("terrain".into())
            .vertices(gen_cube())
            .instance_transforms(terrain::get_transforms(state.pseu_cam.pos))
            .tag(Tags::Terrain)
            .material(MaterialBuilder::default().shader_name("dist".into()).build().unwrap())
            .build()
            .unwrap(),
    );

    state
}
