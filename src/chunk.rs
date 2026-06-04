use crate::lattice;
use noise::{NoiseFn, OpenSimplex, Seedable};

pub const CHUNK_SIZE: usize = 32;

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

    pub fn noise(chunk_x: i32, chunk_z: i32) -> Self {
        const FREQUENCY: f64 = 0.04;
        let noise = OpenSimplex::new(2);
        let mut result = Self::empty();

        for i in 0..CHUNK_SIZE * CHUNK_SIZE {
            let x = i / CHUNK_SIZE;
            let z = i % CHUNK_SIZE;

            let world_x = CHUNK_SIZE as i32 * chunk_x + x as i32;
            let world_z = CHUNK_SIZE as i32 * chunk_z + z as i32;

            let value = noise.get([world_x as f64 * FREQUENCY, world_z as f64 * FREQUENCY]);
            let height = ((value + 1.0) / 2.0 * CHUNK_SIZE as f64) as u32;
            let height = if height == CHUNK_SIZE as u32{
                height - 1
            } else {
                height
            };
            result.blocks[x][z] = (1 << height) - 1;
        }
        return result;
    }

}

impl Default for Chunk {
    fn default() -> Self {
        Self::empty()
    }
}
