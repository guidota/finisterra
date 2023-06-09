// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @builtin(vertex_index) vertex_index : u32,
    @location(0) top_left: vec2<f32>,
    @location(1) bottom_right: vec2<f32>,
    @location(2) tex_top_left: vec2<f32>,
    @location(3) tex_bottom_right: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) z: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>,
    @location(1) color: vec4<f32>,
}

/* @group(2) @binding(0) var<storage, read> vertex_data: array<VertexInput>; */

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    var pos: vec2<f32>;
    var left: f32 = input.top_left.x;
    var right: f32 = input.bottom_right.x;
    var top: f32 = input.top_left.y;
    var bottom: f32 = input.bottom_right.y;

    switch (input.vertex_index) {
        case 0u: {
            pos = vec2<f32>(left, top);
            output.tex_pos = input.tex_top_left;
            break;
        }
        case 1u: {
            pos = vec2<f32>(right, top);
            output.tex_pos = vec2<f32>(input.tex_bottom_right.x, input.tex_top_left.y);
            break;
        }
        case 2u: {
            pos = vec2<f32>(left, bottom);
            output.tex_pos = vec2<f32>(input.tex_top_left.x, input.tex_bottom_right.y);
            break;
        }
        case 3u: {
            pos = vec2<f32>(right, bottom);
            output.tex_pos = input.tex_bottom_right;
            break;
        }
        default: {}
    }

    output.clip_position = camera.view_proj * vec4<f32>(pos, input.z, 1.0);
    output.clip_position.z = input.z;
    output.color = input.color;
    return output;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

fn discard_if_transparent(color: vec4<f32>) {
  if color.w < 0.001 {
    discard;
  }
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var output = textureSample(t_diffuse, s_diffuse, in.tex_pos);
    discard_if_transparent(output);
    return output * in.color;
}
