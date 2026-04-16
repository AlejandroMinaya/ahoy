struct Vertex {
    @location(0) position: vec2u,
};

struct VSOutput {
    @builtin(position) position: vec4f,
    @location(0) enabled: bool
}

const size = 1;
const quad = array(
    vec2f(-1., -1.),
    vec2f(1., -1.),
    vec2f(-1., 1.),
    vec2f(-1., 1.),
    vec2f(1., -1.),
    vec2f(1., 1.),
);

@vertex fn vs(
    v: Vertex,
    @builtin(vertex_index) v_idx: u32,
) -> VSOutput {
    var vs_output: VSOutput;
    vs_output.position = vec4f(v.position + quad[v_idx] * size, 0, 1);
    return vs_output;
}

@fragment fn fs(vs_output: VSOutput) -> @location(0) vec4f {
    return vec4f(0.639216, 0.305882, 0, 1);
}
