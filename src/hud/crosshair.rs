use std::error::Error;

use glium::implement_vertex;
use glium::index::PrimitiveType;
use glium::program;
use glium::uniform;
use glium::Blend;
use glium::{Display, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};

use crate::graphics::{Mesh, Vertex2d, Renderable};

const CROSSHAIR_LENGTH: f32 = 2.0;
const CROSSHAIR_THICKNESS: f32 = 0.5;
const CROSSHAIR_SCALE: f32 = 0.01;




pub struct Crosshair {
    mesh: Mesh<Vertex2d>,
    shader: Program,
    aspect_ratio: f32,
}

impl Crosshair {
    fn create_mesh(display: &Display) -> Mesh<Vertex2d> {


        let verts = &[
            Vertex2d {
                position: [-CROSSHAIR_THICKNESS, -CROSSHAIR_THICKNESS],
            }, // center bot left
            Vertex2d {
                position: [-CROSSHAIR_THICKNESS, CROSSHAIR_THICKNESS],
            }, // center top left
            Vertex2d {
                position: [CROSSHAIR_THICKNESS, -CROSSHAIR_THICKNESS],
            }, // center bot right
            Vertex2d {
                position: [CROSSHAIR_THICKNESS, CROSSHAIR_THICKNESS],
            }, // center top right
            Vertex2d {
                position: [-CROSSHAIR_LENGTH, -CROSSHAIR_THICKNESS],
            }, // left bot
            Vertex2d {
                position: [-CROSSHAIR_LENGTH, CROSSHAIR_THICKNESS],
            }, // left top
            Vertex2d {
                position: [CROSSHAIR_LENGTH, -CROSSHAIR_THICKNESS],
            }, // right bot
            Vertex2d {
                position: [CROSSHAIR_LENGTH, CROSSHAIR_THICKNESS],
            }, // right top
            Vertex2d {
                position: [-CROSSHAIR_THICKNESS, CROSSHAIR_LENGTH],
            }, // top left
            Vertex2d {
                position: [CROSSHAIR_THICKNESS, CROSSHAIR_LENGTH],
            }, // top right
            Vertex2d {
                position: [-CROSSHAIR_THICKNESS, -CROSSHAIR_LENGTH],
            }, // bot left
            Vertex2d {
                position: [CROSSHAIR_THICKNESS, -CROSSHAIR_LENGTH],
            }, // bot right
        ];
        let indices = &[
            0, 1, 2, // center square
            2, 3, 1, 0, 4, 5, // left
            0, 1, 5, 2, 3, 7, // right
            7, 6, 2, 1, 3, 9, // top
            9, 8, 1, 0, 2, 11, 11, 10, 0,
        ];

        Mesh {
            vertices: VertexBuffer::new(display, verts).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap(),
        }
    }

    fn create_shader(display: &Display) -> Program {
        program!(display,
        420 => {
            vertex: include_str!("../shaders/crosshair_shader.vert"),
            fragment: include_str!("../shaders/crosshair_shader.frag"),
        })
        .unwrap()
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn new(display: &Display) -> Self {
        Self {
            mesh: Self::create_mesh(display),
            shader: Self::create_shader(display),
            aspect_ratio: 0.0, // will be overwritten on rendering
        }
    }
}

impl<S: Surface> Renderable<S> for Crosshair {
    fn render(&self, surface: &mut S) -> Result<(), Box<dyn Error>> {

        let uniforms = uniform! {
            aspect_ratio: self.aspect_ratio,
            scale: CROSSHAIR_SCALE,
        };

        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false, // we don't want to corrupt our depth texture with UI
                ..Default::default()
            },
            blend: Blend::alpha_blending(),
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