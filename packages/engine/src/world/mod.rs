pub mod coordinate;

pub const CHUNK_SIZE: usize = 32;

pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
