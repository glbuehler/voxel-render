#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalsUniform {
    pub proj_view_mat: [[f32; 4]; 4],
    pub cam_dir: [f32; 3],
    pub _padding: u32,
}
