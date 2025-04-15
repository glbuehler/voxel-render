use cgmath::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub position: Point3<f32>,
    pub pitch: Rad<f32>,
    pub yaw: Rad<f32>,

    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub zfar: f32,
    pub znear: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            pitch: Rad(0.0),
            yaw: Rad(0.0),

            aspect,
            fovy: Rad(1.0),
            zfar: 100.0,
            znear: 0.1,
        }
    }

    pub fn direction(&self) -> Vector3<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        Vector3::new(cos_pitch * sin_yaw, sin_pitch, -cos_pitch * cos_yaw).normalize()
    }

    pub fn forward(&self) -> Vector3<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        Vector3::new(cos_pitch * sin_yaw, 0.0, -cos_pitch * cos_yaw).normalize()
    }

    pub fn proj_view_matrix(&self) -> Matrix4<f32> {
        let view_mat = Matrix4::look_to_rh(self.position, self.direction(), Vector3::unit_y());
        let proj_mat = cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj_mat * view_mat
    }
}
