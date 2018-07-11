use caper::input::{Input, Key};
use caper::types::Camera;
use caper::utils::build_fp_view_matrix;

use std::f32::consts::PI;

const TWO_PI: f32 = PI * 2f32;
const HALF_PI: f32 = PI / 2f32;

/// This method is where data transforms take place due to inputs
/// for a first person camera
pub fn handle_inputs(input: &mut Input, cam: &mut Camera, delta: f32) {
    // some static vals to use the fp inputs
    let move_speed = 20f32 * delta;
    let mouse_speed: f32 = 30f32 * delta;

    let mv_matrix = build_fp_view_matrix(cam);

    if input.keys_down.contains(&Key::W) {
        cam.pos.0 -= mv_matrix[0][2] * move_speed;
        cam.pos.1 -= mv_matrix[1][2] * move_speed;
        cam.pos.2 -= mv_matrix[2][2] * move_speed;
    }

    if input.keys_down.contains(&Key::D) {
        cam.pos.0 += mv_matrix[0][0] * move_speed;
        cam.pos.1 += mv_matrix[1][0] * move_speed;
        cam.pos.2 += mv_matrix[2][0] * move_speed;
    }

    if input.keys_down.contains(&Key::A) {
        cam.pos.0 -= mv_matrix[0][0] * move_speed;
        cam.pos.1 -= mv_matrix[1][0] * move_speed;
        cam.pos.2 -= mv_matrix[2][0] * move_speed;
    }

    cam.euler_rot.0 += input.mouse_axis_motion.1 * mouse_speed;
    cam.euler_rot.1 += input.mouse_axis_motion.0 * mouse_speed;

    cam.euler_rot.0 = clamp_rot(cam.euler_rot.0);
    cam.euler_rot.1 = clamp_rot(cam.euler_rot.1);

    fn clamp_rot(num: f32) -> f32 {
        let num = if num < 0f32 {
            return TWO_PI - num;
        } else {

            num % TWO_PI
        };

        if num < PI {
            num.min(HALF_PI)
        } else {
            num.max(PI + HALF_PI)
        }
    }
}
