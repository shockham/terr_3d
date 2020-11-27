use caper::glium::Display;
use caper::shader::*;

pub fn add_game_shaders(display: &Display, shaders: &mut Shaders) {
    let _ = shaders.add_shader(
        display,
        "skydome",
        default::gl330::VERT,
        skydome::FRAG,
        default::gl330::GEOM,
        default::gl330::TESS_CONTROL,
        default::gl330::TESS_EVAL,
    );
    let _ = shaders.add_shader(
        display,
        "asteroids",
        default::gl330::VERT,
        asteroids::FRAG,
        default::gl330::GEOM,
        default::gl330::TESS_CONTROL,
        default::gl330::TESS_EVAL,
    );
}

mod skydome {
    // fragment shader
    pub const FRAG: &'static str = "
        #version 330

        #define M_PI 3.1415926535897932384626433832795

        uniform vec3 cam_pos;
        uniform float time;

        in vec3 g_normal;
        in vec3 g_pos;
        in vec2 g_texture;

        out vec4 frag_output;

        float rand (vec3 s) {
            s = round(s * 10);
            return fract(sin(dot(s * 100, vec3(12.9898, 78.233, 54.1232))) * 4.5453);
        }

        void main() {
            float fade_val = sin(-(g_pos.y / 200.0));
            frag_output = vec4(vec3(fade_val), rand(g_pos * 0.01 + sin(time)) * 2.0);
        }";
}

pub mod asteroids {
    /// Distance fragment shader that rolls off to white the further from the camera
    pub const FRAG: &str = "
        #version 330

        uniform vec3 cam_pos;
        uniform sampler1D dir_lights;
        uniform float time;

        in vec3 g_normal;
        in vec3 g_pos;

        out vec4 frag_output;

        float rand (vec3 s) {
            s = round(s * 25);
            return fract(sin(dot(s * 100, vec3(12.9898, 78.233, 54.1232))) * 4.5453);
        }

        void main() {
            int size = textureSize(dir_lights, 0);
            float lum = 0.0;
            for (int i = 0; i < size; i++) {
                vec3 light_norm = normalize(texture(dir_lights, i).xyz);
                lum += max(dot(normalize(g_normal), light_norm), 0.0);
            }

            float dist = abs(distance(cam_pos, g_pos)) / 80.0;

            vec3 color = vec3(0.5 + (0.2 * lum) - (0.6 * dist));
            frag_output = vec4(color, rand(g_pos * 0.01 + sin(time)) * 1.0);
        }
    ";
}
