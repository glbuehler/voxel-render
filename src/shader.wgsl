const XZ: u32 = 64;
const Y: u32 = 64;

const NUM_XZ_VERTICES: u32 = XZ * 4;
const NUM_Y_VERTICES: u32 = Y * 4;

const epsilon: f32 = 0.001;

struct GlobalsUniform {
    proj_view_mat: mat4x4<f32>,
    cam_dir: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> globals: GlobalsUniform;

@group(0) @binding(1)
var<uniform> chunk: array<vec4<u32>, 256>;

const AXIS_X: u32 = 0;
const AXIS_Y: u32 = 1;
const AXIS_Z: u32 = 2;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) axis: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) axis: u32,
}

fn get_block(coord: vec3<f32>) -> bool {
    if (coord.x < -epsilon
        || coord.x > 32.0 + epsilon
        || coord.y < -epsilon
        || coord.y > 32.0 + epsilon
        || coord.z < -epsilon
        || coord.z > 32.0 + epsilon)
    {
        return false;
    }
    let i = u32(coord.x - epsilon) * 32 + u32(coord.z - epsilon);
    let col = chunk[i / 4][i % 4];
    let mask = 1u << u32(coord.y - epsilon);
    return (col & mask) != 0;
}

@vertex
fn vx_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = globals.proj_view_mat * vec4<f32>(in.pos, 1.0);
    out.world_pos = in.pos;
    out.axis = in.axis;
    return out;
}

struct FragmentInput {
    @location(0) world_pos: vec3<f32>,
    @location(1) axis: u32,
}

@fragment
fn fg_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let block = get_block(in.world_pos);
    if block {
        return vec4<f32>(1.0);
    }
    discard;
}
