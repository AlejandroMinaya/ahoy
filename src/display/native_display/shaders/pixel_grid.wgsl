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
    let pixel_coordinates = vec2f(f32(v.x), f32(v.y)) - dimensions;

    let normalized_position = vec2f(pixel_coordinates.x/dimensions.x, pixel_coordinates.y/dimensions.y);
    let normalized_quad_position = vec2f(quad[v_idx].x/dimensions.x, quad[v_idx].y/dimensions.y);

    vs_output.position = vec4f(normalized_position + normalized_quad_position, 1, 1);
    vs_output.enabled = v.enabled;

    return vs_output;

}

@fragment fn fs(vs_output: VSOutput) -> @location(0) vec4f {
    if vs_output.enabled > 0{
        return vec4f(0.639216, 0.305882, 0, 1);
    }
    return vec4f(0);
}
