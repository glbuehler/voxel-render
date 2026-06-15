const XZ: u32 = 128;
const Y: u32 = 64;

const NUM_XZ_VERTICES: u32 = XZ * 4;
const NUM_Y_VERTICES: u32 = Y * 4;

const AXIS_X: u32 = 0;
const AXIS_Y: u32 = 1;
const AXIS_Z: u32 = 2;

const FACE_FRONT: bool = false;
const FACE_BACK: bool = true;

const CHUNK_SIZE: u32 = 32;
const CHUNK_STRIDE: u32 = XZ / CHUNK_SIZE;
const CHUNK_COUNT: u32 = CHUNK_STRIDE * CHUNK_STRIDE;
const CHUNK_ARRAY_SIZE: u32 = CHUNK_COUNT * CHUNK_SIZE * CHUNK_SIZE / 4;

const epsilon: f32 = 0.001;

const light_unnormalized: vec3<f32> = vec3<f32>(0.2, -1.0, 0.3);
const light_norm: f32 = sqrt(
    light_unnormalized.x * light_unnormalized.x
    + light_unnormalized.y * light_unnormalized.y
    + light_unnormalized.z * light_unnormalized.z
);
const light: vec3<f32> = light_unnormalized / light_norm;

struct GlobalsUniform {
    proj_view_mat: mat4x4<f32>,
    cam_pos: vec3<f32>,
    cam_dir: vec3<f32>,
    grid_lines: u32,
}

@group(0) @binding(0)
var<uniform> globals: GlobalsUniform;

@group(0) @binding(1)
var blocks: texture_3d<u32>;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) axis: u32,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) axis: u32,
}

fn get_block(coord: vec3<f32>, axis: u32, face: bool) -> bool {
    let offset = select(0.0, -1.0, face == FACE_BACK);
    var coord_i: vec3<i32>;

    if axis == AXIS_X {
        coord_i.x = i32(round(coord.x + offset));
        coord_i.y = i32(floor(coord.y));
        coord_i.z = i32(floor(coord.z));
    } else if axis == AXIS_Y {
        coord_i.x = i32(floor(coord.x));
        coord_i.y = i32(round(coord.y + offset));
        coord_i.z = i32(floor(coord.z));
    } else {
        coord_i.x = i32(floor(coord.x));
        coord_i.y = i32(floor(coord.y));
        coord_i.z = i32(round(coord.z + offset));
    }

    if coord_i.x < -i32(XZ / 2) || coord_i.x >= i32(XZ)
        || coord_i.z < -i32(XZ / 2) || coord_i.z >= i32(XZ)
        || coord_i.y < -i32(XZ / 2) || coord_i.y >= i32(Y)
    {
        return false;
    }

    let tx = textureLoad(
        blocks,
        vec3<i32>(coord_i.x + i32(XZ / 2), coord_i.z + i32(XZ / 2), coord_i.y + i32(Y / 2)),
        0
    );
    return bool(tx[0]);
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

fn block_color(coord: vec3<f32>, axis: u32, face: bool) -> vec4<f32> {
    let block = get_block(coord, axis, face);
    if !block {
        return vec4<f32>(0.0);;
    }

    var normal: vec3<f32>;
    if axis == AXIS_X {
        normal = vec3<f32>(1.0, 0.0, 0.0);
    } else if axis == AXIS_Y {
        normal = vec3<f32>(0.0, 1.0, 0.0);
    } else {
        normal = vec3<f32>(0.0, 0.0, 1.0);
    }
    if face == FACE_BACK {
        normal *= -1.0;
    }

    let exposed = !get_block(coord - normal, axis, face);
    if !exposed {
        return vec4<f32>(0.0);;
    }

    let dot = dot(light, normal) + 0.3;
    return vec4<f32>(vec3<f32>(dot), 1.0);
}

fn chunk_grid(coord: vec3<f32>, axis: u32, face: bool) -> bool {
    return (
        axis == AXIS_X
        && abs(coord.x % f32(CHUNK_SIZE)) < epsilon
        && (
            abs(coord.z % 4) < epsilon * 32
            || abs(coord.y % 4) < epsilon * 32
        )
    ) || (
        axis == AXIS_Z
        && abs(coord.z % f32(CHUNK_SIZE)) < epsilon
        && (
            abs(coord.x % 4) < epsilon * 32
            || abs(coord.y % 4) < epsilon * 32
        )

    );
}

@fragment
fn fg_main(in: FragmentInput) -> @location(0) vec4<f32> {

    let face = select(FACE_FRONT, FACE_BACK,
        (in.axis == AXIS_X && in.world_pos.x < globals.cam_pos.x)
        || (in.axis == AXIS_Y && in.world_pos.y < globals.cam_pos.y)
        || (in.axis == AXIS_Z && in.world_pos.z < globals.cam_pos.z)
    );

    let block_color = block_color(in.world_pos, in.axis, face);

    if globals.grid_lines != 0 && chunk_grid(in.world_pos, in.axis, face) {
        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }

    if block_color.w > 0.0 {
        return block_color;
    }
    discard;
}
