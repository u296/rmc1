use glium::{Display, Surface, VertexBuffer, IndexBuffer, Program, DrawParameters};
use glium::index::PrimitiveType;
use glium::Blend;
use glium::implement_vertex;
use glium::program;
use glium::uniform;

use crate::graphics::Mesh;

#[derive(Clone, Copy, Debug)]
struct CrosshairVertex {
    position: [f32; 2]
}
implement_vertex!(CrosshairVertex, position);

pub struct Hud {
    crosshair_mesh: Mesh<CrosshairVertex>,
    crosshair_shader: Program,
}

impl Hud {
    fn create_crosshair_mesh(display: &Display) -> Mesh<CrosshairVertex> {
        let length: f32 = 2.0;
        let thickness: f32 = 0.5;

        let verts = &[
            CrosshairVertex{position: [-thickness, -thickness]}, // center bot left
            CrosshairVertex{position: [-thickness,  thickness]}, // center top left
            CrosshairVertex{position: [ thickness, -thickness]}, // center bot right
            CrosshairVertex{position: [ thickness,  thickness]}, // center top right

            CrosshairVertex{position: [-length, -thickness]}, // left bot
            CrosshairVertex{position: [-length,  thickness]}, // left top

            CrosshairVertex{position: [ length, -thickness]}, // right bot
            CrosshairVertex{position: [ length,  thickness]}, // right top

            CrosshairVertex{position: [-thickness,  length]}, // top left
            CrosshairVertex{position: [ thickness,  length]}, // top right

            CrosshairVertex{position: [-thickness, -length]}, // bot left
            CrosshairVertex{position: [ thickness, -length]}, // bot right
        ];
        let indices = &[
            0,1,2, // center square
            2,3,1,

            0,4,5, // left
            0,1,5,

            2,3,7, // right
            7,6,2,

            1,3,9, // top
            9,8,1,

            0,2,11,
            11,10,0,
        ];

        Mesh {
            vertices: VertexBuffer::new(display, verts).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap()
        }
    }

    fn create_crosshair_shader(display: &Display) -> Program {
        program!(display,
        420 => {
            vertex: include_str!("../shaders/crosshair_shader.vert"),
            fragment: include_str!("../shaders/crosshair_shader.frag"),
        }).unwrap()
    }

    pub fn new(display: &Display) -> Self {
        Hud {
            crosshair_mesh: Self::create_crosshair_mesh(display),
            crosshair_shader: Self::create_crosshair_shader(display)
        }
    }

    pub fn render(&self, frame: &mut impl Surface, aspect_ratio: f32) {
        let uniforms = uniform!{
            aspect_ratio: aspect_ratio,
            scale: 0.01f32,
        };

        let params = DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false, // we don't want to corrupt our depth texture with UI
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        frame.draw(&self.crosshair_mesh.vertices, &self.crosshair_mesh.indices, &self.crosshair_shader, &uniforms, &params).unwrap();
    }
}