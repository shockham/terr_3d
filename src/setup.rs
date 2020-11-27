use super::Tags;
use caper::game::*;
use caper::mesh::gen_cube;
use caper::types::{MaterialBuilder, RenderItemBuilder, TextItemBuilder};
use caper::posteffect::PostShaderOptionsBuilder;
use caper::utils::create_skydome;
use state::State;
use terrain;
use terrain::{HALF_MAP_SIZE, MAP_SIZE, SCALE};
use shaders::add_game_shaders;

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
        // add some shaders
        add_game_shaders(&self.renderer.display, &mut self.renderer.shaders);
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
                .name("terrain")
                .vertices(gen_cube())
                .instance_transforms(terrain::get_transforms(state.pseu_cam.pos))
                .tag(Tags::Terrain)
                .active(false)
                .material(
                    MaterialBuilder::default()
                        .shader_name("asteroids")
                        .build()
                        .unwrap(),
                )
                .build()
                .unwrap(),
        );
        self.add_render_item(create_skydome("skydome"));

        self.add_text_item(
            TextItemBuilder::default()
                .text("")
                .pos((-0.35f32, 0f32, 0f32))
                .scale((10f32, 10f32, 1f32))
                .color((0.2f32, 0f32, 0f32, 1f32))
                .build()
                .unwrap(),
        );

        self.add_text_item(
            TextItemBuilder::default()
                .text("")
                .pos((-1f32, -1f32, 0f32))
                .scale((5f32, 5f32, 1f32))
                .color((0.2f32, 0f32, 0f32, 1f32))
                .build()
                .unwrap(),
        );

        self.renderer.post_effect.post_shader_options = PostShaderOptionsBuilder::default()
            .chrom_amt(1f32)
            .blur_amt(5f32)
            .blur_radius(2f32)
            .bokeh(true)
            .bokeh_focal_depth(0.45f32)
            .bokeh_focal_width(0.4f32)
            .color_offset((1f32, 1f32, 1f32, 1f32))
            .build()
            .unwrap();

        state
    }
}
