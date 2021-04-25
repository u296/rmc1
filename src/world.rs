use std::rc::Rc;
use std::cell::RefCell;
use std::io;
use std::fs;
use std::path::Path;


use crate::chunk::*;
use crate::block::types::*;
use crate::camera::{Camera};
use crate::graphics::*;
use crate::terraingen::TerrainGenerator;

use log::*;

use glium::*;
use glium::texture::*;
use glium::index::PrimitiveType;
use glium::uniforms::{Sampler, MinifySamplerFilter, MagnifySamplerFilter};
use glium::framebuffer::*;

#[derive(Debug, Clone, Copy)]
struct SkyVertex {
    position: [f32; 3]
}
implement_vertex!(SkyVertex, position);


const SKY_SHADER_VERT: &'static str = include_str!("shaders/sky_shader.vert");
const SKY_SHADER_FRAG: &'static str = include_str!("shaders/sky_shader.frag");
const CHUNK_SHADER_VERT: &'static str = include_str!("shaders/chunk/vertex.vert");
const CHUNK_COLOR_SHADER_FRAG: &'static str = include_str!("shaders/chunk/color.frag");

const TEXTURE_ATLAS: &'static [u8] = include_bytes!("../atlas.png");



pub struct World {
    pub camera: Camera,
    pub chunks: Vec<Chunk>,
    chunk_meshes: Vec<[ChunkMesh; 2]>, // [0] is normal chunkmesh [1] is transparent chunkmesh
    dirty_chunkmeshes: Vec<usize>, // indices of the chunkmeshes that need to be rebuilt

    chunk_color_shader: Program,

    texture_atlas: CompressedSrgbTexture2d,
    sky_shader: Program,
    sky_mesh: Mesh<SkyVertex>,
}

impl World {
    fn create_sky_mesh(display: &Display) -> Mesh<SkyVertex> {
        let sky_verts_data = &[
            SkyVertex {position: [-1.0, -1.0, -1.0]},
            SkyVertex {position: [-1.0, -1.0,  1.0]},
            SkyVertex {position: [-1.0,  1.0, -1.0]},
            SkyVertex {position: [-1.0,  1.0,  1.0]},
            SkyVertex {position: [ 1.0, -1.0, -1.0]},
            SkyVertex {position: [ 1.0, -1.0,  1.0]},
            SkyVertex {position: [ 1.0,  1.0, -1.0]},
            SkyVertex {position: [ 1.0,  1.0,  1.0]},
        ];

        let sky_indices_data = &[
            0u32,1,2,
            2,3,1,

            4,5,6,
            5,6,7,

            0,1,4,
            4,5,1,

            2,3,6,
            6,7,3,

            0,2,4,
            4,6,2,

            1,3,5,
            5,7,3,
        ];

        Mesh {
            vertices: VertexBuffer::new(display, sky_verts_data).unwrap(),
            indices: IndexBuffer::new(display, PrimitiveType::TrianglesList, sky_indices_data).unwrap(),
        }
    }

    fn shader_helper<P: AsRef<Path>>(path: P, default: &str) -> String {
        match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                error!("{}", e);
                default.into()
            }
        }
    }

    fn texture_helper<P: AsRef<Path>>(path: P, default: &[u8]) -> Vec<u8> {
        match fs::read(path) {
            Ok(b) => b,
            Err(e) => {
                error!("{}", e);
                default.into()
            }
        }
    }

    fn create_sky_shader(display: &Display) -> Result<Program, Box<dyn std::error::Error>> {
        
        Ok(program!(display,
            420 => {
                vertex: &Self::shader_helper("shaders/sky_shader.vert", SKY_SHADER_VERT),
                fragment: &Self::shader_helper("shaders/sky_shader.frag", SKY_SHADER_FRAG)
            }
        )?)
    }

    fn create_chunk_color_shader(display: &Display) -> Result<Program, Box<dyn std::error::Error>> {
        Ok(program! (display,
            420 => {
                vertex: &Self::shader_helper("shaders/chunk/vertex.vert", CHUNK_SHADER_VERT),
                fragment: &Self::shader_helper("shaders/chunk/color.frag", CHUNK_COLOR_SHADER_FRAG)
            }
        )?)
    }

    fn create_texture_atlas(display: &Display) -> Result<CompressedSrgbTexture2d, Box<dyn std::error::Error>> {
        let image = image::load(std::io::Cursor::new(&Self::texture_helper("atlas.png", TEXTURE_ATLAS)[..]), image::ImageFormat::Png)?.to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        Ok(glium::texture::CompressedSrgbTexture2d::new(display, image)?)
    }

    pub fn generate(
        display: &Display,
        generator: &dyn TerrainGenerator,
        width: usize,
        depth: usize,
    ) -> World {
        info!("generating world");

        let mut heightmap: Vec<Vec<u32>>= vec![vec![0; depth]; width];
        

        for (x, rows) in heightmap.iter_mut().enumerate() {
            for (z, height) in rows.iter_mut().enumerate() {
                *height = generator.get_height_at((x as isize, z as isize)) as u32;
            }
        }

        let required_chunk_width = width / CHUNK_SIZE;
        let required_chunk_depth = depth / CHUNK_SIZE;

        let mut chunks = vec![];

        trace!("assigning blocktypes");
        for chunk_x in 0..required_chunk_width {
            for chunk_z in 0..required_chunk_depth {
                for chunk_y in 0..16 {

                    let mut blocktypes = Box::new([[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]);

                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {

                                let global_height: u32 = (CHUNK_SIZE * chunk_y + y) as u32;
                                let global_x = CHUNK_SIZE * chunk_x + x;
                                let global_z = CHUNK_SIZE * chunk_z + z;

                                blocktypes[x][y][z] = {
                                    if global_height <= heightmap[global_x][global_z] - 4 {
                                        Some(&STONE_BLOCK)
                                    } else if global_height <= heightmap[global_x][global_z] - 1 {
                                        Some(&DIRT_BLOCK)
                                    } else if global_height == heightmap[global_x][global_z] {
                                        Some(&GRASS_BLOCK)
                                    } else {
                                        None
                                    }
                                }
                            }
                        }
                    }

                    trace!("creating chunk");
                    let chunk = Box::new(Chunk::from_blocktypes(
                        [chunk_x as i32, chunk_y as i32, chunk_z as i32],
                        &blocktypes
                    ));

                    trace!("pushing chunk");
                    chunks.push(*chunk);
                }
            }
        }

        trace!("generating empty chunkmeshes");
        let mut chunkmeshes = vec![];
        for _ in 0..chunks.len() {
            chunkmeshes.push([ChunkMesh::ungenerated(), ChunkMesh::ungenerated()]);
        }
        let dirty_chunkmeshes = (0..chunks.len()).collect();

        info!("finished generating world");


        let window_size = display.gl_window().window().inner_size();


        World {
            camera: Camera::new(display, [(width / 2) as f32, heightmap[0][0] as f32, (depth / 2) as f32], [0.0, 0.0, 0.0], 90.0, 16.0/9.0),
            chunks: chunks,
            chunk_meshes: chunkmeshes,
            dirty_chunkmeshes: dirty_chunkmeshes,
            chunk_color_shader: Self::create_chunk_color_shader(display).unwrap(),
            texture_atlas: Self::create_texture_atlas(display).unwrap(),
            sky_shader: Self::create_sky_shader(display).unwrap(),
            sky_mesh: Self::create_sky_mesh(display),
        }
    }

    

    pub fn flag_chunkmesh_dirty(&mut self, chunk_coords: [i32; 3]) {

        let index = match self.chunks.iter().position(|c| c.coordinates == chunk_coords) {
            Some(i) => i,
            None => {
                warn!("tried to dirty a nonexistent chunkmesh");
                return;
            }
        };

        self.dirty_chunkmeshes.push(index);
    }

    fn regenerate_dirty_chunkmeshes(display: &Display, chunks: &[Chunk], chunk_meshes: &mut [[ChunkMesh; 2]], dirty_meshes: &mut Vec<usize>, max_regens: usize) {
        debug!("there are {} dirty meshes", dirty_meshes.len());

        let mut num_processed = 0;

        while num_processed < max_regens && dirty_meshes.len() != 0 {
            let i = dirty_meshes[0];

            let chunk = &chunks[i];
            let neighbors = get_chunk_neighbours(chunks, chunk.coordinates);

            if neighbors.is_xz_complete() {
                num_processed += 1;
                chunk_meshes[i][0] = chunk.generate_base_chunkmesh(display, neighbors);
                chunk_meshes[i][1] = chunk.generate_transparent_chunkmesh(display, neighbors);
            }
            
            dirty_meshes.remove(0);
        }
    }

    pub fn update(&mut self, display: &Display, seconds: f32) {
        trace!("updating after {}s", seconds);
        //self.camera.update(seconds);
        
        if !self.dirty_chunkmeshes.is_empty() {
            Self::regenerate_dirty_chunkmeshes(display, &self.chunks, &mut self.chunk_meshes, &mut self.dirty_chunkmeshes, 2);
        }
    }

    pub fn render(&mut self, display: &Display, frame: &mut impl Surface) {
        
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        let color_only_overwrite_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::Overwrite,
                write: false,
                .. Default::default()
            },
            .. Default::default()
        };

        let transparent_params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: false, // we don't want to prevent further away transparent surfaces 
                //from being drawn because a transparent surface is blocking the direct view
                .. Default::default()
            },
            blend: Blend::alpha_blending(),
            .. Default::default()
        };

        let sky_uniforms = uniform! {
            view_rotation: self.camera.get_view_rot(),
            projection: self.camera.get_perspective(),
        };

        frame.draw(
            &self.sky_mesh.vertices,
            &self.sky_mesh.indices,
            &self.sky_shader,
            &sky_uniforms,
            &color_only_overwrite_params
        ).unwrap();

        let worlduniforms = WorldUniforms {
            texture_atlas: self.texture_atlas.sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest),
            render_distance: crate::camera::CLIP_FAR,
        };

        for (chunk, chunk_mesh) in self.chunks.iter().zip(self.chunk_meshes.iter()) {
            chunk_mesh[0].render(
                frame,
                &self.chunk_color_shader,
                &params,
                &worlduniforms,
                &chunk.get_uniforms(),
                &self.camera
            );
        }

        for (chunk, chunk_mesh) in self.chunks.iter().zip(self.chunk_meshes.iter()) {
            chunk_mesh[1].render(
                frame,
                &self.chunk_color_shader,
                &transparent_params,
                &worlduniforms,
                &chunk.get_uniforms(),
                &self.camera
            );
        }


        self.camera.draw_hud(frame);
    }

    pub fn reload_assets(&mut self, display: &Display) -> Result<(), Box<dyn std::error::Error>> {
       

        self.chunk_color_shader = Self::create_chunk_color_shader(display)?;
        self.texture_atlas = Self::create_texture_atlas(display)?;
        self.sky_shader = Self::create_sky_shader(display)?;

        Ok(())
    }
}