use noise::{NoiseFn, Perlin, Seedable};

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

    pub fn random() -> Self {
        let perlin = Perlin::new(1);
        let mut result = Self::empty();

        for i in 0..CHUNK_SIZE * CHUNK_SIZE {
            let x = i / CHUNK_SIZE;
            let z = i % CHUNK_SIZE;
            let noise = perlin.get([x as f64 / CHUNK_SIZE as f64, z as f64 / CHUNK_SIZE as f64]);
            result.blocks[x][z] = (1 << ((noise + 1.0) * CHUNK_SIZE as f64 / 2.0) as u32) - 1;
        }
        return result;
    }
}
