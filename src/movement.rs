use caper::utils::build_fp_view_matrix;
use caper::input::{Key, Input};
use caper::types::Camera;

use std::f32::consts::PI;


/// This method is where data transforms take place due to inputs
/// for a first person camera
pub fn handle_inputs(input: &mut Input, cam: &mut Camera) {
    // some static vals to use the fp inputs
    const MOVE_SPEED: f32 = 1f32;
    const MOUSE_SPEED: f32 = 1f32;
    const TWO_PI: f32 = PI * 2f32;

    let mv_matrix = build_fp_view_matrix(cam);

    if input.keys_down.contains(&Key::S) {
        cam.pos.0 += mv_matrix[0][2] * MOVE_SPEED;
        cam.pos.1 += mv_matrix[1][2] * MOVE_SPEED;
        cam.pos.2 += mv_matrix[2][2] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::W) {
        cam.pos.0 -= mv_matrix[0][2] * MOVE_SPEED;
        cam.pos.1 -= mv_matrix[1][2] * MOVE_SPEED;
        cam.pos.2 -= mv_matrix[2][2] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::D) {
        cam.pos.0 += mv_matrix[0][0] * MOVE_SPEED;
        cam.pos.1 += mv_matrix[1][0] * MOVE_SPEED;
        cam.pos.2 += mv_matrix[2][0] * MOVE_SPEED;
    }

    if input.keys_down.contains(&Key::A) {
        cam.pos.0 -= mv_matrix[0][0] * MOVE_SPEED;
        cam.pos.1 -= mv_matrix[1][0] * MOVE_SPEED;
        cam.pos.2 -= mv_matrix[2][0] * MOVE_SPEED;
    }

    cam.euler_rot.0 += input.mouse_axis_motion.1 * MOUSE_SPEED;
    cam.euler_rot.1 += input.mouse_axis_motion.0 * MOUSE_SPEED;

    cam.euler_rot.0 = fix_rot(cam.euler_rot.0);
    cam.euler_rot.1 = fix_rot(cam.euler_rot.1);

    // make sure euler_rot always between 0 and 2PI
    fn fix_rot(num: f32) -> f32 {
        if num < 0f32 {
            return TWO_PI - num;
        }

        num % TWO_PI
    }
}
