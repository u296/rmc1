use glium::implement_vertex;
use glium::index::*;
use glium::texture::*;
use glium::uniforms::Sampler;
use glium::vertex::*;
use glium::Surface;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex2d {
    pub position: [f32; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex3d {
    pub position: [f32; 3],
}

implement_vertex!(Vertex, position, uv, normal);
implement_vertex!(Vertex2d, position);
implement_vertex!(Vertex3d, position);

pub struct Mesh<V: Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>,
}

/// uniforms that are shared by multiple chunks
#[derive(Clone, Copy)]
pub struct WorldUniforms<'a> {
    pub texture_atlas: Sampler<'a, CompressedSrgbTexture2d>,
    pub render_distance: f32,
}

pub trait Renderable<S: Surface> {
    fn render(&self, surface: &mut S) -> Result<(), Box<dyn std::error::Error>>;
}