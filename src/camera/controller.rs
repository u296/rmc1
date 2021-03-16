use log::trace;

use super::Camera;

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
    pub fn update_camera(&self, camera: &mut Camera, seconds: f32) {
        let forward = camera.get_view_dir();
        let right = camera.get_right_dir();

        let right = {
            let len = (right[0].powi(2) + right[1].powi(2) + right[2].powi(2)).sqrt();
            [right[0] / len, right[1] / len, right[2] / len]
        };

        if self.moving_up {
            camera.position[1] +=  seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
            trace!("moving up");
        }

        if self.moving_down {
            camera.position[1] -= seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
        }

        if self.moving_left {
            camera.position[0] -= right[0] * seconds * MOVESPEED;
            camera.position[1] -= right[1] * seconds * MOVESPEED;
            camera.position[2] -= right[2] * seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
        }

        if self.moving_right {
            camera.position[0] += right[0] * seconds * MOVESPEED;
            camera.position[1] += right[1] * seconds * MOVESPEED;
            camera.position[2] += right[2] * seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
        }

        if self.moving_forward {
            camera.position[0] += forward[0] * seconds * MOVESPEED;
            camera.position[1] += forward[1] * seconds * MOVESPEED;
            camera.position[2] += forward[2] * seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
        }

        if self.moving_back {
            camera.position[0] -= forward[0] * seconds * MOVESPEED;
            camera.position[1] -= forward[1] * seconds * MOVESPEED;
            camera.position[2] -= forward[2] * seconds * MOVESPEED;
            camera.values.borrow_mut().dirty = true;
        }

        trace!("forward: {:?}", forward);
        trace!("position: {:?}", camera.position)
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