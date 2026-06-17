#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlobalsUniform {
    pub proj_view_mat: [[f32; 4]; 4],
    pub light_mat: [[f32; 4]; 4],
    pub cam_pos: [f32; 3],
    pub cam_dir: [f32; 3],
    pub light_dir: [f32; 3],
    pub _pad0: u32,
    pub _pad1: u32,
    pub grid_lines: u32,
}
