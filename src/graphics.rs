use glium::implement_vertex;
use glium::index::*;
use glium::vertex::*;
use glium::texture::*;
use glium::uniforms::Sampler;
use glium::framebuffer::*;
use glium::Display;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex2d {
    pub position: [f32; 2]
}

implement_vertex!(Vertex, position, uv, normal);
implement_vertex!(Vertex2d, position);

pub struct Mesh<V: Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>
}


/// uniforms that are shared by multiple chunks
#[derive(Clone, Copy)]
pub struct WorldUniforms<'a> {
    pub texture_atlas: Sampler<'a, CompressedSrgbTexture2d>,
    pub render_distance: f32,
}


pub struct WorldRenderData {
    pub depth_buffer: DepthRenderBuffer,
    pub frag_colors: Texture2d,
    pub frag_depths: Texture2d,
    pub frag_normals: Texture2d,

    pub fb_quad: Mesh<Vertex2d>,
}

impl WorldRenderData {
    pub fn new(display: &Display) -> Self {
        let fbquad_mesh = {
            let fbquad_verts_data = &[
                Vertex2d{ position: [-1.0, -1.0]},
                Vertex2d{ position: [ 1.0, -1.0]},
                Vertex2d{ position: [-1.0,  1.0]},
                Vertex2d{ position: [ 1.0,  1.0]},
            ];
    
            let fbquad_indices_data = &[
                0, 1, 2,
                1, 2, 3
            ];
            
            Mesh {
                vertices: VertexBuffer::new(display, fbquad_verts_data).unwrap(),
                indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, fbquad_indices_data).unwrap(),
            }
        };

        let size = display.gl_window().window().inner_size();
        WorldRenderData {
            depth_buffer: DepthRenderBuffer::new(display, DepthFormat::F32, size.width, size.height).unwrap(),
            frag_colors: Texture2d::empty(display, size.width, size.height).unwrap(),
            frag_depths: Texture2d::empty(display, size.width, size.height).unwrap(),
            frag_normals: Texture2d::empty(display, size.width, size.height).unwrap(),
            fb_quad: fbquad_mesh,
        }
    }

    pub fn create_color_framebuffer(&self, display: &Display) -> SimpleFrameBuffer {
        SimpleFrameBuffer::with_depth_buffer(display, &self.frag_colors, &self.depth_buffer).unwrap()
    }

    pub fn create_depth_framebuffer(&self, display: &Display) -> SimpleFrameBuffer {
        SimpleFrameBuffer::with_depth_buffer(display, &self.frag_depths, &self.depth_buffer).unwrap()
    }

    pub fn create_normal_framebuffer(&self, display: &Display) -> SimpleFrameBuffer {
        SimpleFrameBuffer::with_depth_buffer(display, &self.frag_normals, &self.depth_buffer).unwrap()
    }

    pub fn remake(&mut self, display: &Display) {
        let size = display.gl_window().window().inner_size();

        self.depth_buffer = DepthRenderBuffer::new(display, DepthFormat::F32, size.width, size.height).unwrap();
        self.frag_colors = Texture2d::empty(display, size.width, size.height).unwrap();
        self.frag_depths = Texture2d::empty(display, size.width, size.height).unwrap();
        self.frag_normals = Texture2d::empty(display, size.width, size.height).unwrap();
    }
}