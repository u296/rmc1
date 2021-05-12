use std::cell::RefCell;
use std::f32::consts::TAU;

use super::Camera;
use super::{CLIP_FAR, CLIP_NEAR};

#[derive(Default)]
struct FirstPersonCameraCache {
    dirty: bool,
    perspective_matrix: [[f32; 4]; 4],
    view_rotation_matrix: [[f32; 4]; 4],
    inverse_view_rotation_matrix: [[f32; 4]; 4],
    view_dir: [f32; 3],
    right_dir: [f32; 3],
    view_translation_matrix: [[f32; 4]; 4],
}

impl FirstPersonCameraCache {
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn recalculate(&mut self, position: [f32; 3], rotation: [f32; 3], fov: f32, aspect_ratio: f32) {
        let fov = fov * TAU;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        self.perspective_matrix = [
            [f / aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (CLIP_FAR + CLIP_NEAR) / (CLIP_FAR - CLIP_NEAR),
                1.0,
            ],
            [
                0.0,
                0.0,
                -(2.0 * CLIP_FAR * CLIP_NEAR) / (CLIP_FAR - CLIP_NEAR),
                0.0,
            ],
        ];

        // TAU to convert to radians, negative to rotate clockwise
        let (sy, cy) = (TAU * -rotation[0]).sin_cos();
        let (sp, cp) = (TAU * -rotation[1]).sin_cos();
        //let (sr, cr) = (TAU * -rotation[2]).sin_cos();

        self.view_rotation_matrix = [
            [cy, sp * sy, -sy * cp, 0.0],
            [0.0, cp, sp, 0.0],
            [sy, -cy * sp, cy * cp, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let (sy, cy) = (TAU * rotation[0]).sin_cos();
        let (sp, cp) = (TAU * rotation[1]).sin_cos();

        self.inverse_view_rotation_matrix = [
            [cy, 0.0, -sy, 0.0],
            [sp * sy, cp, sp * cy, 0.0],
            [cp * sy, -sp, cp * cy, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        self.view_translation_matrix = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-position[0], -position[1], -position[2], 1.0],
        ];

        let forward = [0.0, 0.0, 1.0];
        let right = [1.0, 0.0, 0.0];

        self.view_dir = [
            forward[0] * self.inverse_view_rotation_matrix[0][0]
                + forward[1] * self.inverse_view_rotation_matrix[1][0]
                + forward[2] * self.inverse_view_rotation_matrix[2][0],
            forward[0] * self.inverse_view_rotation_matrix[0][1]
                + forward[1] * self.inverse_view_rotation_matrix[1][1]
                + forward[2] * self.inverse_view_rotation_matrix[2][1],
            forward[0] * self.inverse_view_rotation_matrix[0][2]
                + forward[1] * self.inverse_view_rotation_matrix[1][2]
                + forward[2] * self.inverse_view_rotation_matrix[2][2],
        ];

        self.right_dir = [
            right[0] * self.inverse_view_rotation_matrix[0][0]
                + right[1] * self.inverse_view_rotation_matrix[1][0]
                + right[2] * self.inverse_view_rotation_matrix[2][0],
            right[0] * self.inverse_view_rotation_matrix[0][1]
                + right[1] * self.inverse_view_rotation_matrix[1][1]
                + right[2] * self.inverse_view_rotation_matrix[2][1],
            right[0] * self.inverse_view_rotation_matrix[0][2]
                + right[1] * self.inverse_view_rotation_matrix[1][2]
                + right[2] * self.inverse_view_rotation_matrix[2][2],
        ];

        self.dirty = false;
    }
}

// all angles are in turns
pub struct FirstPersonCamera {
    position: [f32; 3],
    rotation: [f32; 3],
    fov: f32,
    aspect_ratio: f32,
    values: RefCell<FirstPersonCameraCache>,
}

impl FirstPersonCamera {
    pub fn new(
        pos: [f32; 3],
        rotation: [f32; 3],
        fov: f32,
        aspect_ratio: f32,
    ) -> FirstPersonCamera {
        FirstPersonCamera {
            position: pos,
            rotation: rotation,
            fov: fov,
            aspect_ratio: aspect_ratio,
            values: RefCell::new(FirstPersonCameraCache {
                dirty: true,
                ..Default::default()
            }),
        }
    }
}

impl Camera for FirstPersonCamera {
    fn get_position(&self) -> &[f32; 3] {
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut [f32; 3] {
        self.values.borrow_mut().dirty = true;
        &mut self.position
    }

    fn get_rotation(&self) -> &[f32; 3] {
        &self.rotation
    }

    fn get_rotation_mut(&mut self) -> &mut [f32; 3] {
        self.values.borrow_mut().dirty = true;
        &mut self.rotation
    }

    fn get_aspect_ratio(&self) -> &f32 {
        &self.aspect_ratio
    }

    fn get_aspect_ratio_mut(&mut self) -> &mut f32 {
        self.values.borrow_mut().dirty = true;
        &mut self.aspect_ratio
    }

    fn get_projection(&self) -> [[f32; 4]; 4] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().perspective_matrix
    }

    fn get_view_rotation(&self) -> [[f32; 4]; 4] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().view_rotation_matrix
    }

    fn get_view_translation(&self) -> [[f32; 4]; 4] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().view_translation_matrix
    }

    fn get_forward_direction(&self) -> [f32; 3] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().view_dir
    }

    fn get_right_direction(&self) -> [f32; 3] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().right_dir
    }
}
