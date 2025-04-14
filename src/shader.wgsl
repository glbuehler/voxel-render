
@group(0) @binding(0)
var<uniform> proj_view_mat: mat4x4<f32>;

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) col: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) col: vec3<f32>,
}


@vertex
fn vx_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = proj_view_mat * vec4<f32>(in.pos, 1.0);
    out.world_pos = in.pos;
    out.col = in.col;
    return out;
}

struct FragmentInput {
    @location(0) world_pos: vec3<f32>,
    @location(1) col: vec3<f32>,
}

@fragment
fn fg_main(in: FragmentInput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.col, 1.0);
}
