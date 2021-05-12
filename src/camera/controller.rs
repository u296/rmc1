use log::trace;

use crate::camera::Camera;

const MOVESPEED: f32 = 10.0;

pub struct Controller {
    pub moving_forward: bool,
    pub moving_back: bool,
    pub moving_left: bool,
    pub moving_right: bool,
    pub moving_up: bool,
    pub moving_down: bool,
}

impl Controller {
    pub fn update_camera(&self, camera: &mut dyn Camera, seconds: f32) {
        let forward = camera.get_forward_direction();
        let right = camera.get_right_direction();

        let right = {
            let len = (right[0].powi(2) + right[1].powi(2) + right[2].powi(2)).sqrt();
            [right[0] / len, right[1] / len, right[2] / len]
        };

        let cam_pos = camera.get_position_mut();

        if self.moving_up {
            cam_pos[1] += seconds * MOVESPEED;
            trace!("moving up");
        }

        if self.moving_down {
            cam_pos[1] -= seconds * MOVESPEED;
        }

        if self.moving_left {
            cam_pos[0] -= right[0] * seconds * MOVESPEED;
            cam_pos[1] -= right[1] * seconds * MOVESPEED;
            cam_pos[2] -= right[2] * seconds * MOVESPEED;
        }

        if self.moving_right {
            cam_pos[0] += right[0] * seconds * MOVESPEED;
            cam_pos[1] += right[1] * seconds * MOVESPEED;
            cam_pos[2] += right[2] * seconds * MOVESPEED;
        }

        if self.moving_forward {
            cam_pos[0] += forward[0] * seconds * MOVESPEED;
            cam_pos[1] += forward[1] * seconds * MOVESPEED;
            cam_pos[2] += forward[2] * seconds * MOVESPEED;
        }

        if self.moving_back {
            cam_pos[0] -= forward[0] * seconds * MOVESPEED;
            cam_pos[1] -= forward[1] * seconds * MOVESPEED;
            cam_pos[2] -= forward[2] * seconds * MOVESPEED;
        }

        trace!("forward: {:?}", forward);
        trace!("position: {:?}", cam_pos)
    }
}

impl Default for Controller {
    fn default() -> Self {
        Controller {
            moving_forward: false,
            moving_back: false,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
        }
    }
}
