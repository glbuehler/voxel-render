
struct BackgroundUniform {
    resolution: vec2<u32>,
    millis_elapsed: u32,
    pitch: f32,
    yaw: f32,
    fovy: f32,
};

@group(0) @binding(0)
var<uniform> uniform: BackgroundUniform;

@vertex
fn vx_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

fn hash3(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3<f32>(12.9898, 78.233, 45.164))) * 43758.5453);
}

// Simple rotation around X
fn rotate_x(v: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(
        v.x,
        c * v.y - s * v.z,
        s * v.y + c * v.z
    );
}

// Simple rotation around Y
fn rotate_y(v: vec3<f32>, angle: f32) -> vec3<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec3<f32>(
        c * v.x + s * v.z,
        v.y,
        -s * v.x + c * v.z
    );
}

fn twinkle(seed: f32, time: f32) -> f32 {
    let freq = mix(1.5, 4.5, fract(seed * 13.37));
    let phase = fract(seed * 73.1) * 6.2831; // full 0–2π phase shift
    return 0.5 + 0.5 * sin(time * freq + phase);
}

@fragment
fn fg_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let time = f32(uniform.millis_elapsed) * 0.0005;
    let uv = (frag_coord.xy / vec2<f32>(uniform.resolution)) * 2.0 - 1.0;
    let aspect = f32(uniform.resolution.x) / f32(uniform.resolution.y);
    let fov_adjust = tan(uniform.fovy / 2.0);

    var dir = normalize(vec3<f32>(
        uv.x * aspect * fov_adjust,
        -uv.y * fov_adjust,
        1.0
    ));

    dir = rotate_x(dir, uniform.pitch);
    dir = rotate_y(dir, uniform.yaw);

    let chunk = floor(dir * 100.0);

    let flicker = twinkle(hash3(chunk), time + hash3(floor(dir * 100.0 + 23.0)) * 10.0);
    let stars = smoothstep(
        0.997,
        1.0,
        hash3(chunk)
    ) * flicker;

    return vec4<f32>(vec3<f32>(stars), 1.0);
}
