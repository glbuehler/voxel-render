use crate::chunk::{self, CHUNK_SIZE};
use crate::lattice;

pub const CHUNK_XZ: usize = lattice::XZ as usize / CHUNK_SIZE;
pub const CHUNK_Y: usize = lattice::Y as usize / CHUNK_SIZE;
pub const CHUNK_COUNT: usize = CHUNK_XZ * CHUNK_XZ * CHUNK_Y;

const CHUNK_BUFFER_SIZE: usize = (lattice::XZ * lattice::XZ * lattice::Y / u32::BITS) as usize;

type ChunkRenderBuffer = [u32; CHUNK_BUFFER_SIZE];

pub struct ChunkStorage {
    chunks: [chunk::Chunk; CHUNK_COUNT],
}

impl ChunkStorage {
    pub fn new() -> Self {
        let mut chunks = [chunk::Chunk::default(); CHUNK_COUNT];
        chunks[0] = chunk::Chunk::noise(0, 0);
        chunks[1] = chunk::Chunk::noise(0, 1);
        chunks[2] = chunk::Chunk::noise(1, 0);
        chunks[3] = chunk::Chunk::noise(1, 1);

        Self { chunks }
    }

    pub fn get(&self, x: isize, y: isize, z: isize) -> Option<&chunk::Chunk> {
        if x < 0 || y < 0 || z < 0 {
            return None;
        }
        let (x, y, z) = (x as usize, y as usize, z as usize);
        self.chunks.get(y * CHUNK_Y * CHUNK_XZ + x * CHUNK_XZ + z)
    }

    pub fn copy_to_render_buffer(&self, x: isize, y: isize, z: isize) -> ChunkRenderBuffer {
        let mut buf: ChunkRenderBuffer = [0; _];

        for chunk_z in 0..CHUNK_XZ as usize {
            for chunk_x in 0..CHUNK_XZ as usize {
                for chunk_y in 0..CHUNK_Y as usize {
                    let Some(chunk) = self.get(
                        chunk_x as isize + x,
                        chunk_y as isize + y,
                        chunk_z as isize + z,
                    ) else {
                        continue;
                    };
                    let idx = chunk_x * CHUNK_SIZE
                        + chunk_z * CHUNK_SIZE * CHUNK_XZ
                        + chunk_y * CHUNK_SIZE * CHUNK_XZ * CHUNK_XZ;

                    for z in 0..CHUNK_SIZE {
                        for x in 0..CHUNK_SIZE {
                            buf[idx + x + z * CHUNK_SIZE] = chunk.blocks[x][z];
                        }
                    }
                }
            }
        }
        buf
    }
}
