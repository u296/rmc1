pub mod controller;
mod hud;

use std::cell::RefCell;

use glium::{Display, Surface};

use hud::Hud;

pub const CLIP_NEAR: f32 = 0.01;
pub const CLIP_FAR: f32 = 64.0;

#[derive(Default)]
struct Values {
    dirty: bool,
    perspective_matrix: [[f32; 4]; 4],
    view_rotation_matrix: [[f32; 4]; 4],
    inverse_view_rotation_matrix: [[f32; 4]; 4],
    view_dir: [f32; 3],
    right_dir: [f32; 3],
    view_translation_matrix: [[f32; 4]; 4],
}

impl Values {
    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn recalculate(&mut self, position: [f32; 3], rotation: [f32; 3], fov: f32, aspect_ratio: f32) {
        let fov = fov.to_radians();

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

        let (sy, cy) = rotation[0].sin_cos();
        let (sp, cp) = rotation[1].sin_cos();
        //let (sr, cr) = self.rotation.2.sin_cos();

        self.view_rotation_matrix = [
            [cy, sp * sy, -sy * cp, 0.0],
            [0.0, cp, sp, 0.0],
            [sy, -cy * sp, cy * cp, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let (sy, cy) = (-rotation[0]).sin_cos();
        let (sp, cp) = (-rotation[1]).sin_cos();
        //let (sr, cr) = (-self.rotation.2).sin_cos();

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

pub struct Camera {
    position: [f32; 3],
    rotation: [f32; 3], // yaw pitch roll (radians)
    fov: f32,           // in degrees
    aspect_ratio: f32,
    hud: Hud,
    values: RefCell<Values>,
}

impl Camera {
    pub fn new(
        display: &Display,
        pos: [f32; 3],
        rotation: [f32; 3],
        fov: f32,
        aspect_ratio: f32,
    ) -> Camera {
        Camera {
            position: pos,
            rotation: rotation,
            fov: fov,
            aspect_ratio: aspect_ratio,
            hud: Hud::new(display),
            values: RefCell::new(Values {
                dirty: true,
                ..Default::default()
            }),
        }
    }

    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.values.borrow_mut().dirty = true;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
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

    pub fn get_view_rot(&self) -> [[f32; 4]; 4] {
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

    pub fn get_inverse_view_rot(&self) -> [[f32; 4]; 4] {
        if self.values.borrow().is_dirty() {
            self.values.borrow_mut().recalculate(
                self.position,
                self.rotation,
                self.fov,
                self.aspect_ratio,
            );
        }

        self.values.borrow().inverse_view_rotation_matrix
    }

    pub fn get_view_translation(&self) -> [[f32; 4]; 4] {
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

    pub fn rotate_yaw(&mut self, degrees: f32) {
        self.rotation[0] += degrees.to_radians();
        self.values.borrow_mut().dirty = true;
    }

    pub fn rotate_pitch(&mut self, degrees: f32) {
        self.rotation[1] += degrees.to_radians();
        self.values.borrow_mut().dirty = true;
    }

    pub fn get_view_dir(&self) -> [f32; 3] {
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

    pub fn get_right_dir(&self) -> [f32; 3] {
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

    pub fn draw_hud(&self, frame: &mut impl Surface) {
        self.hud.render(frame, self.aspect_ratio);
    }
}
