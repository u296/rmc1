use std::error::Error;
use glium::{Display, Surface, Program, VertexBuffer, IndexBuffer, index::PrimitiveType, uniform};
use crate::graphics::*;

const SKY_SHADER_VERT: &'static str = include_str!("../shaders/sky_shader.vert");
const SKY_SHADER_FRAG: &'static str = include_str!("../shaders/sky_shader.frag");

const SKY_SHADER_HOTLOAD_VERT: &'static str = "shaders/sky_shader.vert";
const SKY_SHADER_HOTLOAD_FRAG: &'static str = "shaders/sky_shader.frag";

pub struct Sky {
    mesh: Mesh<Vertex3d>,
    shader: Program,

    view_rotation: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}

impl Sky {
    fn create_mesh(display: &Display) -> Mesh<Vertex3d> {
        let sky_verts_data = &[
            Vertex3d {
                position: [-1.0, -1.0, -1.0],
            },
            Vertex3d {
                position: [-1.0, -1.0, 1.0],
            },
            Vertex3d {
                position: [-1.0, 1.0, -1.0],
            },
            Vertex3d {
                position: [-1.0, 1.0, 1.0],
            },
            Vertex3d {
                position: [1.0, -1.0, -1.0],
            },
            Vertex3d {
                position: [1.0, -1.0, 1.0],
            },
            Vertex3d {
                position: [1.0, 1.0, -1.0],
            },
            Vertex3d {
                position: [1.0, 1.0, 1.0],
            },
        ];

        let sky_indices_data = &[
            0u32, 1, 2, 2, 3, 1, 4, 5, 6, 5, 6, 7, 0, 1, 4, 4, 5, 1, 2, 3, 6, 6, 7, 3, 0, 2, 4, 4,
            6, 2, 1, 3, 5, 5, 7, 3,
        ];

        Mesh {
            vertices: VertexBuffer::new(display, sky_verts_data).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, sky_indices_data)
                .unwrap(),
        }
    }

    fn create_shader(display: &Display) -> Program {
        super::shader_load_helper(display, SKY_SHADER_HOTLOAD_VERT, SKY_SHADER_HOTLOAD_FRAG, SKY_SHADER_VERT, SKY_SHADER_FRAG)
    }

    pub fn set_view_rotation(&mut self, view_rotation: [[f32; 4]; 4]) {
        self.view_rotation = view_rotation;
    }

    pub fn set_projection(&mut self, projection: [[f32; 4]; 4]) {
        self.projection = projection;
    }

    pub fn reload(&mut self, display: &Display) {
        self.shader = Self::create_shader(display);
    }

    pub fn new(display: &Display) -> Self {
        Self {
            mesh: Self::create_mesh(display),
            shader: Self::create_shader(display),

            view_rotation: Default::default(),
            projection: Default::default(),
        }
    }
}

impl<S: Surface> Renderable<S> for Sky {
    fn render(&self, surface: &mut S) -> Result<(), Box<dyn Error>> {
        let uniforms = uniform! {
            view_rotation: self.view_rotation,
            projection: self.projection,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false,
                ..Default::default()
            },
            ..Default::default()
        };

        match surface.draw(
            &self.mesh.vertices,
            &self.mesh.indices,
            &self.shader,
            &uniforms,
            &params
        ) {
            Err(e) => Err(Box::new(e)),
            _ => Ok(())
        }
    }
}