use caper::input::Key;
use caper::types::Camera;
use caper::utils::build_fp_view_matrix;
use caper::game::Game;
use super::Tags;

use std::f32::consts::PI;

const HALF_PI: f32 = PI / 2f32;

pub trait HandleInput {
    fn handle_inputs(&mut self, cam: &mut Camera);
}

impl HandleInput for Game<Tags> {
    /// This method is where data transforms take place due to inputs
    /// for a first person camera
    fn handle_inputs(&mut self, cam: &mut Camera) {
        if self.input.hide_mouse {
            // some static vals to use the fp inputs
            let move_speed = 15f32 * self.delta;
            let mouse_speed: f32 = 30f32 * self.delta;

            let mv_matrix = build_fp_view_matrix(cam);

            if self.input.keys_down.contains(&Key::W) {
                cam.pos.0 -= mv_matrix[0][2] * move_speed;
                cam.pos.1 -= mv_matrix[1][2] * move_speed;
                cam.pos.2 -= mv_matrix[2][2] * move_speed;
            }

            if self.input.keys_down.contains(&Key::D) {
                cam.pos.0 += mv_matrix[0][0] * move_speed;
                cam.pos.1 += mv_matrix[1][0] * move_speed;
                cam.pos.2 += mv_matrix[2][0] * move_speed;
            }

            if self.input.keys_down.contains(&Key::A) {
                cam.pos.0 -= mv_matrix[0][0] * move_speed;
                cam.pos.1 -= mv_matrix[1][0] * move_speed;
                cam.pos.2 -= mv_matrix[2][0] * move_speed;
            }

            cam.euler_rot.0 += self.input.mouse_axis_motion.1 * mouse_speed;
            cam.euler_rot.1 += self.input.mouse_axis_motion.0 * mouse_speed;

            cam.euler_rot.0 = clamp_rot(cam.euler_rot.0);
            cam.euler_rot.1 = clamp_rot(cam.euler_rot.1);

            // set cam rot to be that for pseu_cam
            self.cams[0].euler_rot = cam.euler_rot;

            fn clamp_rot(num: f32) -> f32 {
                num.max(-HALF_PI).min(HALF_PI)
            }
        }
    }
}
