use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub col: [f32; 3],
}

impl Vertex {
    pub fn stride() -> u64 {
        std::mem::size_of::<Self>() as u64
    }

    pub fn attributes() -> [wgpu::VertexAttribute; 2] {
        wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x3,
        ]
    }
}
