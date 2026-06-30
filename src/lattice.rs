use crate::vertex::{self, Vertex};
use cgmath;

pub const XZ: u32 = 128;
pub const Y: u32 = 64;
pub const NUM_VERTICES: usize = (8 * (XZ + 1) + 4 * (Y + 1)) as usize;
pub const NUM_INDICES: usize = (12 * (XZ + 1) + 6 * (Y + 1)) as usize;

pub const MAX_X: i32 = (XZ as i32) / 2 - 1;
pub const MAX_Z: i32 = (XZ as i32) / 2 - 1;
pub const MAX_Y: i32 = (Y as i32) / 2 - 1;
pub const MIN_X: i32 = -(XZ as i32) / 2;
pub const MIN_Z: i32 = -(XZ as i32) / 2;
pub const MIN_Y: i32 = -(Y as i32) / 2;

pub const CORNERS: &[cgmath::Vector3<f32>] = &[
    cgmath::Vector3 {
        x: MIN_X as f32,
        y: MIN_Y as f32,
        z: MIN_Z as f32,
    },
    cgmath::Vector3 {
        x: MAX_X as f32,
        y: MIN_Y as f32,
        z: MIN_Z as f32,
    },
    cgmath::Vector3 {
        x: MIN_X as f32,
        y: MAX_Y as f32,
        z: MIN_Z as f32,
    },
    cgmath::Vector3 {
        x: MAX_X as f32,
        y: MAX_Y as f32,
        z: MIN_Z as f32,
    },
    cgmath::Vector3 {
        x: MIN_X as f32,
        y: MIN_Y as f32,
        z: MAX_Z as f32,
    },
    cgmath::Vector3 {
        x: MAX_X as f32,
        y: MIN_Y as f32,
        z: MAX_Z as f32,
    },
    cgmath::Vector3 {
        x: MIN_X as f32,
        y: MAX_Y as f32,
        z: MAX_Z as f32,
    },
    cgmath::Vector3 {
        x: MAX_X as f32,
        y: MAX_Y as f32,
        z: MAX_Z as f32,
    },
];

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Lattice {
    pub vertices: [Vertex; NUM_VERTICES],
    pub indices: [u16; NUM_INDICES],
}

impl Lattice {
    pub const fn new() -> Self {
        let mut s = Self {
            vertices: [Vertex {
                pos: [0.0, 0.0, 0.0],
                axis: 0,
            }; NUM_VERTICES],
            indices: [0; NUM_INDICES],
        };

        let mut v = 0;
        let mut i = 0;
        let mut xz = 0;
        while xz <= XZ {
            s.vertices[v + 0] = Vertex {
                pos: [
                    xz as f32 - (XZ / 2) as f32,
                    -(Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_X,
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    xz as f32 - (XZ / 2) as f32,
                    -(Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_X,
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    xz as f32 - (XZ / 2) as f32,
                    (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_X,
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    xz as f32 - (XZ / 2) as f32,
                    (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_X,
            };
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;

            v += 4;
            i += 6;

            s.vertices[v + 0] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    -(Y as f32 / 2.0),
                    xz as f32 - (XZ / 2) as f32,
                ],
                axis: vertex::AXIS_Z,
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    (Y as f32 / 2.0),
                    xz as f32 - (XZ / 2) as f32,
                ],
                axis: vertex::AXIS_Z,
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    -(Y as f32 / 2.0),
                    xz as f32 - (XZ / 2) as f32,
                ],
                axis: vertex::AXIS_Z,
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    (Y as f32 / 2.0),
                    xz as f32 - (XZ / 2) as f32,
                ],
                axis: vertex::AXIS_Z,
            };
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;

            xz += 1;
            v += 4;
            i += 6;
        }

        let mut y = 0;
        while y <= Y {
            s.vertices[v + 0] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_Y,
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_Y,
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_Y,
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
                axis: vertex::AXIS_Y,
            };
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;

            y += 1;
            v += 4;
            i += 6;
        }
        s
    }
}
