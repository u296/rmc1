pub mod controller;

mod firstperson;
mod orbital;

pub use firstperson::FirstPersonCamera;
pub use orbital::OrbitalCamera;

pub const CLIP_NEAR: f32 = 0.01;
pub const CLIP_FAR: f32 = 64.0;

pub trait Camera {
    // these return matrices that manipulate the world around the camera
    fn get_view_translation(&self) -> [[f32; 4]; 4];
    fn get_view_rotation(&self) -> [[f32; 4]; 4];
    fn get_projection(&self) -> [[f32; 4]; 4];

    fn get_position(&self) -> &[f32; 3];
    fn get_position_mut(&mut self) -> &mut [f32; 3];

    fn get_rotation(&self) -> &[f32; 3];
    fn get_rotation_mut(&mut self) -> &mut [f32; 3];

    fn get_aspect_ratio(&self) -> &f32;
    fn get_aspect_ratio_mut(&mut self) -> &mut f32;

    fn get_forward_direction(&self) -> [f32; 3];
    fn get_right_direction(&self) -> [f32; 3];
}
