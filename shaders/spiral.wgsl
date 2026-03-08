// Spiral visualization with Musical Window (40 Hz–18 kHz).
// Vertex attribute: position (vec2), t (f32). Discard bins outside 0 <= t <= 1.

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) t: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) t: f32,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Move out-of-window vertices off-screen so they are not drawn (t < 0 or t > 1).
    let discard_vertex = in.t < 0.0 || in.t > 1.0;
    out.clip_position = select(
        vec4(in.position, 0.0, 1.0),
        vec4(2.0, 2.0, 2.0, 1.0),
        discard_vertex
    );
    out.t = in.t;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Visual clean-up: discard any fragment that slipped through (t < 0 or t > 1).
    if (in.t < 0.0 || in.t > 1.0) {
        discard;
    }
    // Your spiral color logic here (e.g. based on in.t or other varyings).
    return vec4(1.0, 1.0, 1.0, 1.0);
}
