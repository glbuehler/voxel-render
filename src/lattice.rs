use crate::vertex::Vertex;

pub const XZ: u32 = 64;
pub const Y: u32 = 64;
pub const NUM_VERTICES: usize = (8 * XZ + 4 * Y) as usize;
pub const NUM_INDICES: usize = 2 * (12 * XZ + 6 * Y) as usize;

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
            }; NUM_VERTICES],
            indices: [0; NUM_INDICES],
        };

        let mut v = 0;
        let mut i = 0;
        let mut x = 0;
        while x < XZ {
            s.vertices[v + 0] = Vertex {
                pos: [
                    x as f32 - (XZ / 2) as f32,
                    -(Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    x as f32 - (XZ / 2) as f32,
                    -(Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    x as f32 - (XZ / 2) as f32,
                    (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    x as f32 - (XZ / 2) as f32,
                    (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
            };
            // ccw
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;
            // cw
            s.indices[i + 6] = v as u16 + 0;
            s.indices[i + 7] = v as u16 + 2;
            s.indices[i + 8] = v as u16 + 1;
            s.indices[i + 9] = v as u16 + 1;
            s.indices[i + 10] = v as u16 + 2;
            s.indices[i + 11] = v as u16 + 3;

            x += 1;
            v += 4;
            i += 12;
        }

        let mut z = 0;
        while z < XZ {
            s.vertices[v + 0] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    -(Y as f32 / 2.0),
                    z as f32 - (XZ / 2) as f32,
                ],
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    (Y as f32 / 2.0),
                    z as f32 - (XZ / 2) as f32,
                ],
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    -(Y as f32 / 2.0),
                    z as f32 - (XZ / 2) as f32,
                ],
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    (Y as f32 / 2.0),
                    z as f32 - (XZ / 2) as f32,
                ],
            };
            // ccw
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;
            // cw
            s.indices[i + 6] = v as u16 + 0;
            s.indices[i + 7] = v as u16 + 2;
            s.indices[i + 8] = v as u16 + 1;
            s.indices[i + 9] = v as u16 + 1;
            s.indices[i + 10] = v as u16 + 2;
            s.indices[i + 11] = v as u16 + 3;

            z += 1;
            v += 4;
            i += 12;
        }

        let mut y = 0;
        while y < Y {
            s.vertices[v + 0] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 1] = Vertex {
                pos: [
                    -(XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 2] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    -(XZ as f32 / 2.0),
                ],
            };
            s.vertices[v + 3] = Vertex {
                pos: [
                    (XZ as f32 / 2.0),
                    y as f32 - (Y as f32 / 2.0),
                    (XZ as f32 / 2.0),
                ],
            };
            // ccw
            s.indices[i + 0] = v as u16 + 0;
            s.indices[i + 1] = v as u16 + 1;
            s.indices[i + 2] = v as u16 + 2;
            s.indices[i + 3] = v as u16 + 1;
            s.indices[i + 4] = v as u16 + 3;
            s.indices[i + 5] = v as u16 + 2;
            // cw
            s.indices[i + 6] = v as u16 + 0;
            s.indices[i + 7] = v as u16 + 2;
            s.indices[i + 8] = v as u16 + 1;
            s.indices[i + 9] = v as u16 + 1;
            s.indices[i + 10] = v as u16 + 2;
            s.indices[i + 11] = v as u16 + 3;

            y += 1;
            v += 4;
            i += 12;
        }
        s
    }
}
