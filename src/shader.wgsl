const XZ: u32 = 64;
const Y: u32 = 64;

const NUM_XZ_VERTICES: u32 = XZ * 4;
const NUM_Y_VERTICES: u32 = Y * 4;

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
    return coord.x == 1.0 && coord.y == 1.0 && coord.z == 1.0;


    // if (coord.x >= 32 || coord.y >= 32 || coord.z >= 32) {
    //     return false;
    // }
    // let i = coord.x * 32 + coord.z;
    // let col = chunk[i / 4][i % 4];
    // let mask = 1u << coord.y;
    // return (col & mask) != 0;
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
    let red = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let dir = globals.cam_dir;
    var block: bool;

    // if (in.axis == AXIS_X) {
    //     block = get_block(world_pos);
    //     if (block) {
    //         return red;
    //     }
    //     return vec4<f32>(0.0);
    // }
    // if (in.axis == AXIS_Y) {
    //     block = get_block(world_pos);
    //     if (block) {
    //         return red;
    //     }
    //     return vec4<f32>(0.0);
    // }

    // block = get_block(in.world_pos);
    if (in.world_pos.x < 1.0 && in.world_pos.x >= 0.0 && in.world_pos.y < 1.0 && in.world_pos.y >= 0.0 && in.world_pos.z < 1.0 && in.world_pos.z >= 0.0) {
        return red;
    }
    return vec4<f32>(0.0);
}
