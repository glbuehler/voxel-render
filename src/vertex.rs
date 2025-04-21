pub const AXIS_X: u32 = 0;
pub const AXIS_Y: u32 = 1;
pub const AXIS_Z: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub axis: u32,
}

impl Vertex {
    pub fn stride() -> u64 {
        std::mem::size_of::<Self>() as u64
    }

    pub fn attributes() -> [wgpu::VertexAttribute; 2] {
        wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Uint32,
        ]
    }
}
