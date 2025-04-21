const CHUNK_SIZE: usize = 32;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Chunk {
    pub blocks: [[u32; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub const fn empty() -> Self {
        Self {
            blocks: [[0; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}
