use caper::types::{Camera, RenderItemBuilder, MaterialBuilder};
use caper::game::*;
use terrain::{HALF_MAP_SIZE, MAP_SIZE, SCALE};
use super::Tags;
use caper::mesh::gen_cube;

use terrain;


/// Setup the game
pub fn setup(game: &mut Game<Tags>, pseu_cam: Camera) {
    game.cams[0].pos = (HALF_MAP_SIZE, HALF_MAP_SIZE, (MAP_SIZE as f32 * SCALE) - 60f32);
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
}
