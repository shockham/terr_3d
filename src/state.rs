use caper::types::{Camera, CameraBuilder};

pub struct State {
    pub pseu_cam: Camera,
    pub alive: bool,
}

impl Default for State {
    fn default() -> State {
        State {
            pseu_cam: CameraBuilder::default().build().unwrap(),
            alive: false,
        }
    }
}
