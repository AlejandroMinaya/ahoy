struct Vertex {
    @location(0) x: u32,
    @location(1) y: u32,
    @location(2) enabled: u32,
};

struct VSOutput {
    @builtin(position) position: vec4f,
    @location(0) enabled: u32
}

const quad = array(
    vec2f(-1., -1.),
    vec2f(1., -1.),
    vec2f(-1., 1.),
    vec2f(-1., 1.),
    vec2f(1., -1.),
    vec2f(1., 1.),
);
const dimensions = vec2f(32., 16.);

@vertex fn vs(
    v: Vertex,
    @builtin(vertex_index) v_idx: u32,
) -> VSOutput {

    var vs_output: VSOutput;
    let pixel_coordinates = vec2f(f32(v.x), f32(v.y)) - dimensions + quad[v_idx];

    let normalized_position = vec2f(pixel_coordinates.x/dimensions.x, pixel_coordinates.y/dimensions.y);

    vs_output.position = vec4f(normalized_position, 1, 1);
    vs_output.enabled = v.enabled;

    return vs_output;

}

@fragment fn fs(vs_output: VSOutput) -> @location(0) vec4f {
    let color = vec4f(0.709804, 0.282353, 0, 1);
    if vs_output.enabled > 0 {
        return color;
    }
    return vec4f(color.xyz * .1, 1);
}
