use super::{BlockType, BlockUVCoordinates};

#[allow(dead_code)]
pub const GRASS_BLOCK: BlockType = BlockType{
    name: "grass block",
    uv: BlockUVCoordinates {
        front: ([0.0, 1.0-1.0/64.0], [1.0/64.0, 1.0]),
        right: ([0.0, 1.0-1.0/64.0], [1.0/64.0, 1.0]),
        back:  ([0.0, 1.0-1.0/64.0], [1.0/64.0, 1.0]),
        left:  ([0.0, 1.0-1.0/64.0], [1.0/64.0, 1.0]),
        top: ([1.0/64.0, 1.0-1.0/64.0], [1.0/32.0, 1.0]),
        bottom: ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
    },
    transparent: false
};

#[allow(dead_code)]
pub const DIRT_BLOCK: BlockType = BlockType{
    name: "dirt block",
    uv: BlockUVCoordinates {
        front:  ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
        right:  ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
        back:   ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
        left:   ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
        top:    ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
        bottom: ([1.0/32.0, 1.0-1.0/64.0], [3.0*(1.0/64.0), 1.0]),
    },
    transparent: false
};

#[allow(dead_code)]
pub const STONE_BLOCK: BlockType = BlockType {
    name: "stone block",
    uv: BlockUVCoordinates {
        front:  ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
        right:  ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
        back:   ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
        left:   ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
        top:    ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
        bottom: ([3.0*(1.0/64.0), 1.0-1.0/64.0], [1.0/16.0, 1.0]),
    },
    transparent: false
};

#[allow(dead_code)]
pub const GLASS_BLOCK: BlockType = BlockType {
    name: "glass block",
    uv: BlockUVCoordinates {
        front:  ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
        right:  ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
        back:   ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
        left:   ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
        top:    ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
        bottom: ([4.0 * (1.0/64.0), 63.0/64.0], [5.0 * (1.0/64.0), 1.0]),
    },
    transparent: true,
};