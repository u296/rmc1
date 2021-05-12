use std::convert::TryInto;

pub use crate::block::*;
use crate::camera::Camera;
use crate::graphics::{Mesh, Vertex, WorldUniforms};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use glium::index::PrimitiveType;
use glium::{uniform, Display, DrawParameters, Program, Surface};

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_SIZE_I8: i8 = 32;
pub const CHUNK_SIZE_U8: u8 = 32;
pub const CHUNK_SIZE_I32: i32 = 32;

pub struct ChunkUniforms {
    model_translation: [[f32; 4]; 4],
    model_rotation: [[f32; 4]; 4],
}

#[derive(Clone, Copy)]
pub struct ChunkNeighbours<'a> {
    pub front: Option<&'a Chunk>,
    pub back: Option<&'a Chunk>,
    pub left: Option<&'a Chunk>,
    pub right: Option<&'a Chunk>,
    pub above: Option<&'a Chunk>,
    pub below: Option<&'a Chunk>,
}

pub fn get_chunk_neighbours<'a>(chunks: &'a [Chunk], coords: [i32; 3]) -> ChunkNeighbours<'a> {
    let front_coords = [coords[0], coords[1], coords[2] + 1];
    let back_coords = [coords[0], coords[1], coords[2] - 1];
    let right_coords = [coords[0] + 1, coords[1], coords[2]];
    let left_coords = [coords[0] - 1, coords[1], coords[2]];
    let above_coords = [coords[0], coords[1] + 1, coords[2]];
    let below_coords = [coords[0], coords[1] - 1, coords[2]];

    ChunkNeighbours {
        front: chunks.iter().find(|c| c.coordinates == front_coords),
        back: chunks.iter().find(|c| c.coordinates == back_coords),
        left: chunks.iter().find(|c| c.coordinates == left_coords),
        right: chunks.iter().find(|c| c.coordinates == right_coords),
        above: chunks.iter().find(|c| c.coordinates == above_coords),
        below: chunks.iter().find(|c| c.coordinates == below_coords),
    }
}

impl<'a> ChunkNeighbours<'a> {
    /*pub fn is_complete(&self) -> bool {
        self.front.is_some()
            && self.back.is_some()
            && self.right.is_some()
            && self.left.is_some()
            && self.above.is_some()
            && self.below.is_some()
    }*/

    pub fn is_xz_complete(&self) -> bool {
        self.front.is_some() && self.back.is_some() && self.right.is_some() && self.left.is_some()
    }
}

pub struct ChunkMesh {
    mesh: Option<Mesh<Vertex>>,
    pub dirty: bool,
}

impl ChunkMesh {
    pub fn ungenerated() -> Self {
        ChunkMesh {
            mesh: None,
            dirty: true,
        }
    }

    pub fn render<S: Surface>(
        &self,
        surface: &mut S,
        shader: &Program,
        params: &DrawParameters,
        world_uniforms: &WorldUniforms,
        chunk_uniforms: &ChunkUniforms,
        camera: &dyn Camera,
    ) {
        match &self.mesh {
            Some(mesh) => {
                let uniforms = uniform! {
                    projection: camera.get_projection(),
                    view_translation: camera.get_view_translation(),
                    view_rotation: camera.get_view_rotation(),
                    model_translation: chunk_uniforms.model_translation,
                    model_rotation: chunk_uniforms.model_rotation,
                    atlas: world_uniforms.texture_atlas,
                    render_distance: world_uniforms.render_distance,
                };

                surface
                    .draw(&mesh.vertices, &mesh.indices, &shader, &uniforms, &params)
                    .unwrap();
            }
            None => (),
        }
    }
}

pub struct Chunk {
    pub coordinates: [i32; 3],
    pub blocks: Box<[[[Option<Block>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
}

impl Chunk {
    #[allow(dead_code)]
    pub fn filled(coords: [i32; 3], block_type: &'static BlockType) -> Chunk {
        let mut blocks: Box<[[[Option<Block>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]> =
            vec![[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap();

        for x in 0..CHUNK_SIZE_U8 {
            for y in 0..CHUNK_SIZE_U8 {
                for z in 0..CHUNK_SIZE_U8 {
                    blocks[x as usize][y as usize][z as usize] =
                        Some(Block::new([x, y, z], block_type));
                }
            }
        }

        Chunk {
            coordinates: coords,
            blocks: blocks,
        }
    }

    pub fn from_blocktypes(
        coords: [i32; 3],
        blocktypes: &[[[Option<&'static BlockType>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    ) -> Self {
        trace!("allocating blocks");
        let mut blocks: Box<[[[Option<Block>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]> =
            vec![[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    blocks[x][y][z] = match blocktypes[x][y][z] {
                        Some(blocktype) => Some(Block::new([x as u8, y as u8, z as u8], blocktype)),
                        None => None,
                    }
                }
            }
        }

        trace!("initializing chunk at {:?}", coords);
        Chunk {
            coordinates: coords,
            blocks: blocks,
        }
    }

    fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [
                (self.coordinates[0] * CHUNK_SIZE_I32) as f32,
                (self.coordinates[1] * CHUNK_SIZE_I32) as f32,
                (self.coordinates[2] * CHUNK_SIZE_I32) as f32,
                1.0,
            ],
        ]
    }

    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    pub fn get_uniforms(&self) -> ChunkUniforms {
        ChunkUniforms {
            model_translation: self.get_translation_matrix(),
            model_rotation: self.get_rotation_matrix(),
        }
    }

    fn get_block_neighbors(
        &self,
        block_position: [u8; 3],
        chunk_neighbors: ChunkNeighbours,
    ) -> BlockNeighbors {
        let mut neighbors = BlockNeighbors {
            right: None,
            left: None,
            above: None,
            below: None,
            back: None,
            front: None,
        };

        let right_block = (
            block_position[0] as i8 + 1,
            block_position[1] as i8,
            block_position[2] as i8,
        );

        let left_block = (
            block_position[0] as i8 - 1,
            block_position[1] as i8,
            block_position[2] as i8,
        );

        let top_block = (
            block_position[0] as i8,
            block_position[1] as i8 + 1,
            block_position[2] as i8,
        );

        let below_block = (
            block_position[0] as i8,
            block_position[1] as i8 - 1,
            block_position[2] as i8,
        );

        let front_block = (
            block_position[0] as i8,
            block_position[1] as i8,
            block_position[2] as i8 + 1,
        );

        let back_block = (
            block_position[0] as i8,
            block_position[1] as i8,
            block_position[2] as i8 - 1,
        );

        // the majority of the blocks checked will be inside
        // the chunk itself, we can improve cache usage by
        // reading from self whenever possible
        let block_checker = |local_block_coord: (i8, i8, i8)| {
            if 0 <= local_block_coord.0
                && local_block_coord.0 < CHUNK_SIZE_I8
                && 0 <= local_block_coord.1
                && local_block_coord.1 < CHUNK_SIZE_I8
                && 0 <= local_block_coord.2
                && local_block_coord.2 < CHUNK_SIZE_I8
            {
                self.blocks[local_block_coord.0 as usize][local_block_coord.1 as usize]
                    [local_block_coord.2 as usize]
            } else {
                if local_block_coord.0 < 0 {
                    return match chunk_neighbors.left {
                        Some(left) => {
                            left.blocks[CHUNK_SIZE - 1][local_block_coord.1 as usize]
                                [local_block_coord.2 as usize]
                        }
                        None => None,
                    };
                }
                if local_block_coord.0 >= CHUNK_SIZE_I8 {
                    return match chunk_neighbors.right {
                        Some(right) => {
                            right.blocks[0][local_block_coord.1 as usize]
                                [local_block_coord.2 as usize]
                        }
                        None => None,
                    };
                }
                if local_block_coord.1 < 0 {
                    return match chunk_neighbors.below {
                        Some(below) => {
                            below.blocks[local_block_coord.0 as usize][CHUNK_SIZE - 1]
                                [local_block_coord.2 as usize]
                        }
                        None => None,
                    };
                }
                if local_block_coord.1 >= CHUNK_SIZE_I8 {
                    return match chunk_neighbors.above {
                        Some(above) => {
                            above.blocks[local_block_coord.0 as usize][0]
                                [local_block_coord.2 as usize]
                        }
                        None => None,
                    };
                }
                if local_block_coord.2 < 0 {
                    return match chunk_neighbors.back {
                        Some(back) => {
                            back.blocks[local_block_coord.0 as usize][local_block_coord.1 as usize]
                                [CHUNK_SIZE - 1]
                        }
                        None => None,
                    };
                }
                if local_block_coord.2 >= CHUNK_SIZE_I8 {
                    return match chunk_neighbors.front {
                        Some(front) => {
                            front.blocks[local_block_coord.0 as usize][local_block_coord.1 as usize]
                                [0]
                        }
                        None => None,
                    };
                }

                panic!()
            }
        };

        neighbors.right = block_checker(right_block);
        neighbors.left = block_checker(left_block);
        neighbors.above = block_checker(top_block);
        neighbors.below = block_checker(below_block);
        neighbors.front = block_checker(front_block);
        neighbors.back = block_checker(back_block);

        neighbors
    }

    pub fn get_block(&self, coords: [u8; 3]) -> &Option<Block> {
        trace!(
            "getting block at {:?} in chunk {:?}",
            coords,
            self.coordinates
        );
        &self.blocks[coords[0] as usize][coords[1] as usize][coords[2] as usize]
    }

    pub fn get_block_mut(&mut self, coords: [u8; 3]) -> &mut Option<Block> {
        trace!(
            "getting block at {:?} in chunk {:?} as mut",
            coords,
            self.coordinates
        );
        &mut self.blocks[coords[0] as usize][coords[1] as usize][coords[2] as usize]
    }

    fn add_face(current: &Block, neighbor: &Option<Block>) -> bool {
        if neighbor.is_none() {
            true
        } else {
            match neighbor.unwrap().block_type.transparent {
                true => {
                    if current.block_type.transparent {
                        false
                    } else {
                        true
                    }
                }
                false => {
                    if current.block_type.transparent {
                        false
                    } else {
                        false
                    }
                }
            }
        }
    }

    pub fn generate_base_chunkmesh(
        &self,
        display: &Display,
        chunk_neighbors: ChunkNeighbours,
    ) -> ChunkMesh {
        self.generate_chunkmesh(display, |t| !t.transparent, chunk_neighbors)
    }

    pub fn generate_transparent_chunkmesh(
        &self,
        display: &Display,
        chunk_neighbors: ChunkNeighbours,
    ) -> ChunkMesh {
        self.generate_chunkmesh(display, |t| t.transparent, chunk_neighbors)
    }

    // block_includer should return true when
    // a block should be included in the mesh
    fn generate_chunkmesh<F: Fn(&BlockType) -> bool>(
        &self,
        display: &glium::Display,
        block_includer: F,
        chunk_neighbors: ChunkNeighbours,
    ) -> ChunkMesh {
        let mut verts: Vec<Vertex> = vec![];
        let mut indices: Vec<u32> = vec![];

        for i in self.blocks.iter() {
            for j in i.iter() {
                for block in j.iter() {
                    match block {
                        None => continue,
                        Some(block) => {
                            if !block_includer(block.block_type) {
                                continue;
                            }

                            let base_indices = &[0u32, 1, 2, 2, 3, 1];

                            let neighbors =
                                self.get_block_neighbors(block.in_chunk_position, chunk_neighbors);

                            let uv = block.block_type.uv;
                            if Self::add_face(&block, &neighbors.right) {
                                let xcoord = (block.in_chunk_position[0] + 1) as f32;
                                let ybase = block.in_chunk_position[1] as f32;
                                let zbase = block.in_chunk_position[2] as f32;

                                let n = [1.0, 0.0, 0.0];
                                let uv = uv.right;

                                let face_verts = &[
                                    Vertex {
                                        position: [xcoord, ybase, zbase],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase + 1.0, zbase],
                                        uv: [uv.1[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase, zbase + 1.0],
                                        uv: [uv.0[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase + 1.0, zbase + 1.0],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }

                            if Self::add_face(&block, &neighbors.left) {
                                let xcoord = block.in_chunk_position[0] as f32;
                                let ybase = block.in_chunk_position[1] as f32;
                                let zbase = block.in_chunk_position[2] as f32;

                                let n = [-1.0, 0.0, 0.0];
                                let uv = uv.left;

                                let face_verts = &[
                                    Vertex {
                                        position: [xcoord, ybase, zbase],
                                        uv: [uv.0[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase + 1.0, zbase],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase, zbase + 1.0],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xcoord, ybase + 1.0, zbase + 1.0],
                                        uv: [uv.1[0], uv.1[1]],
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }

                            if Self::add_face(&block, &neighbors.above) {
                                let xbase = block.in_chunk_position[0] as f32;
                                let ycoord = (block.in_chunk_position[1] + 1) as f32;
                                let zbase = block.in_chunk_position[2] as f32;

                                let n = [0.0, 1.0, 0.0];
                                let uv = uv.top;

                                let face_verts = &[
                                    Vertex {
                                        position: [xbase, ycoord, zbase],
                                        uv: uv.0,
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ycoord, zbase],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase, ycoord, zbase + 1.0],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ycoord, zbase + 1.0],
                                        uv: uv.1,
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }

                            if Self::add_face(&block, &neighbors.below) {
                                let xbase = block.in_chunk_position[0] as f32;
                                let ycoord = block.in_chunk_position[1] as f32;
                                let zbase = block.in_chunk_position[2] as f32;

                                let n = [0.0, -1.0, 0.0];
                                let uv = uv.bottom;

                                let face_verts = &[
                                    Vertex {
                                        position: [xbase, ycoord, zbase],
                                        uv: uv.0,
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ycoord, zbase],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase, ycoord, zbase + 1.0],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ycoord, zbase + 1.0],
                                        uv: uv.1,
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }

                            if Self::add_face(&block, &neighbors.front) {
                                let xbase = block.in_chunk_position[0] as f32;
                                let ybase = block.in_chunk_position[1] as f32;
                                let zcoord = (block.in_chunk_position[2] + 1) as f32;

                                let n = [0.0, 0.0, 1.0];
                                let uv = uv.front;

                                let face_verts = &[
                                    Vertex {
                                        position: [xbase, ybase, zcoord],
                                        uv: uv.0,
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ybase, zcoord],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase, ybase + 1.0, zcoord],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ybase + 1.0, zcoord],
                                        uv: uv.1,
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }

                            if Self::add_face(&block, &neighbors.back) {
                                let xbase = block.in_chunk_position[0] as f32;
                                let ybase = block.in_chunk_position[1] as f32;
                                let zcoord = block.in_chunk_position[2] as f32;

                                let n = [0.0, 0.0, -1.0];
                                let uv = uv.back;

                                let face_verts = &[
                                    Vertex {
                                        position: [xbase, ybase, zcoord],
                                        uv: [uv.1[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ybase, zcoord],
                                        uv: [uv.0[0], uv.0[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase, ybase + 1.0, zcoord],
                                        uv: [uv.1[0], uv.1[1]],
                                        normal: n,
                                    },
                                    Vertex {
                                        position: [xbase + 1.0, ybase + 1.0, zcoord],
                                        uv: [uv.0[0], uv.1[1]],
                                        normal: n,
                                    },
                                ];

                                let offset: u32 = verts.len().try_into().unwrap();

                                let face_indices = base_indices.iter().map(|i| i + offset);

                                verts.extend_from_slice(face_verts);
                                indices.extend(face_indices);
                            }
                        }
                    }
                }
            }
        }

        debug!("mesh contains {} faces", indices.len() / 3);
        if verts.len() == 0 {
            ChunkMesh {
                mesh: None,
                dirty: false,
            }
        } else {
            let verts = glium::vertex::VertexBuffer::new(display, &verts)
                .expect("failed to create vertex buffer");
            let indices =
                glium::index::IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices)
                    .expect("failed to create index buffer");

            ChunkMesh {
                mesh: Some(Mesh {
                    vertices: verts,
                    indices: indices,
                }),
                dirty: false,
            }
        }
    }

    /// computes the chunk and block inside that chunk that a global
    /// coordinate corresponds to
    pub fn get_local_coord_from_world_coord(coord: [i32; 3]) -> ([i32; 3], [u8; 3]) {
        let inchunk = [
            (coord[0] % CHUNK_SIZE_I32) as u8,
            (coord[1] % CHUNK_SIZE_I32) as u8,
            (coord[2] % CHUNK_SIZE_I32) as u8,
        ];

        let chunk = [
            (coord[0] - inchunk[0] as i32) / CHUNK_SIZE_I32,
            (coord[1] - inchunk[1] as i32) / CHUNK_SIZE_I32,
            (coord[2] - inchunk[2] as i32) / CHUNK_SIZE_I32,
        ];

        (chunk, inchunk)
    }

    #[allow(dead_code)]
    pub fn get_global_coords_from_local_coord<B: Into<i32> + Copy>(
        chunk: [i32; 3],
        block: [B; 3],
    ) -> [i32; 3] {
        [
            chunk[0] * CHUNK_SIZE_I32 + block[0].into(),
            chunk[1] * CHUNK_SIZE_I32 + block[1].into(),
            chunk[2] * CHUNK_SIZE_I32 + block[2].into(),
        ]
    }
}
