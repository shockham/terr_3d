use super::Tags;
use caper::game::*;
use caper::mesh::gen_cube;
use caper::types::{MaterialBuilder, RenderItemBuilder};
use state::State;
use terrain;
use terrain::{HALF_MAP_SIZE, MAP_SIZE, SCALE};

/// trait for game setup
pub trait Setup {
    /// State type
    type S: Default;
    /// Setup method
    fn setup(&mut self) -> Self::S;
}

impl Setup for Game<Tags> {
    type S = State;
    /// Setup the game
    fn setup(&mut self) -> Self::S {
        // create init state
        let state = Self::S::default();
        // set the cam pos
        self.cams[0].pos = (
            HALF_MAP_SIZE,
            HALF_MAP_SIZE,
            (MAP_SIZE as f32 * SCALE) - 60f32,
        );
        // define some items to be rendered
        self.add_render_item(
            RenderItemBuilder::default()
                .name("terrain".into())
                .vertices(gen_cube())
                .instance_transforms(terrain::get_transforms(state.pseu_cam.pos))
                .tag(Tags::Terrain)
                .material(
                    MaterialBuilder::default()
                        .shader_name("dist".into())
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );

        state
    }
}
