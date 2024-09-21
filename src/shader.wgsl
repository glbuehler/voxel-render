

@vertex
fn vx_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    let pos = array(
        vec2(0.0, 0.5),
        vec2(-0.5, -0.5),
        vec2(0.5, -0.5)
    );

    return vec4<f32>(-0.5, -0.5, 0.0, 1.0);
}
