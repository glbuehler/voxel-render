const XZ: u32 = 64;
const Y: u32 = 64;

const NUM_XZ_VERTICES: u32 = XZ * 4;
const NUM_Y_VERTICES: u32 = Y * 4;

@group(0) @binding(0)
var<uniform> proj_view_mat: mat4x4<f32>;

struct VertexInput {
    @builtin(vertex_index) idx: u32,
    @location(0) pos: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec3<f32>,
}


@vertex
fn vx_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = proj_view_mat * vec4<f32>(in.pos, 1.0);
    if (in.idx < NUM_XZ_VERTICES) {
        out.col = vec3<f32>(1.0, 0.0, 0.0);
    } else if (in.idx < 2 * NUM_XZ_VERTICES) {
        out.col = vec3<f32>(0.0, 1.0, 0.0);
    } else {
        out.col = vec3<f32>(0.0, 0.0, 1.0);
    }

    return out;
}

struct FragmentInput {
    @location(0) col: vec3<f32>,
}

@fragment
fn fg_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.col, 1.0);
}
