
pub mod types;

#[derive(Copy, Clone, Debug)]
pub struct BlockUVCoordinates {
    pub front: ([f32; 2], [f32; 2]),
    pub right: ([f32; 2], [f32; 2]),
    pub back: ([f32; 2], [f32; 2]),
    pub left: ([f32; 2], [f32; 2]),
    pub top: ([f32; 2], [f32; 2]),
    pub bottom: ([f32; 2], [f32; 2]),
}

#[derive(Copy, Clone, Debug)]
pub struct BlockType {
    pub name: &'static str,
    pub uv: BlockUVCoordinates,
    pub transparent: bool,
}

pub struct BlockNeighbors {
    pub right: Option<Block>,
    pub left: Option<Block>,
    pub above: Option<Block>,
    pub below: Option<Block>,
    pub back: Option<Block>,
    pub front: Option<Block>,
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub in_chunk_position: [u8; 3],
    pub block_type: &'static BlockType,
}

impl Block {
    pub fn new(chunk_offset: [u8; 3], block_type: &'static BlockType) -> Block {
        Block{
            in_chunk_position: chunk_offset.clone(),
            block_type: block_type
        }
    }
}
