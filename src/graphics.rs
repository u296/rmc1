use glium::implement_vertex;
use glium::index::IndexBuffer;
use glium::vertex::VertexBuffer;
use glium::texture::CompressedSrgbTexture2d;
use glium::uniforms::Sampler;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}
implement_vertex!(Vertex, position, uv, normal);


pub struct Mesh<V: Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>
}


/// uniforms that are shared by multiple chunks
#[derive(Clone, Copy)]
pub struct WorldUniforms<'a> {
    pub texture_atlas: Sampler<'a, CompressedSrgbTexture2d>,
}