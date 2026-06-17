use crate::camera::{self, Camera};

pub fn directional(
    dir: cgmath::Vector3<f32>,
    cam_mat: cgmath::Matrix4<f32>,
) -> cgmath::Matrix4<f32> {
    use cgmath::*;

    let view_mat = Matrix4::look_to_rh(Point3::origin(), dir, Vector3::unit_y());
    let inverse = cam_mat.invert().unwrap();

    let mut corners = [
        Vector4::new(-1.0, -1.0, 0.0, 1.0),
        Vector4::new(-1.0, -1.0, 1.0, 1.0),
        Vector4::new(-1.0, 1.0, 0.0, 1.0),
        Vector4::new(-1.0, 1.0, 1.0, 1.0),
        Vector4::new(1.0, -1.0, 0.0, 1.0),
        Vector4::new(1.0, -1.0, 1.0, 1.0),
        Vector4::new(1.0, 1.0, 0.0, 1.0),
        Vector4::new(1.0, 1.0, 1.0, 1.0),
    ];

    corners.iter_mut().for_each(|v| {
        *v = inverse * *v;
        // *v *= v.w;
        *v = view_mat * *v;
    });

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    corners.iter().for_each(|v| {
        min_x = f32::min(min_x, v.x);
        max_x = f32::max(max_x, v.x);
        min_y = f32::min(min_y, v.y);
        max_y = f32::max(max_y, v.y);
        min_z = f32::min(min_z, v.z);
        max_z = f32::max(max_z, v.z);
    });

    let proj_mat = ortho(min_x, max_x, min_y, max_y, min_z, max_z);
    return camera::OPENGL_TO_WGPU_MATRIX * proj_mat * view_mat;
}
