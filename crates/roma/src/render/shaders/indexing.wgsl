struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) index: i32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) index: i32,
}

@vertex
fn vert_main(vertex: VertexInput) -> VertexOutput {
    var outval: VertexOutput;
    outval.position = camera.view_proj * vec4<f32>(vertex.position.x, vertex.position.y, 0.0, 1.0);
    outval.tex_coord = vertex.tex_coord;
    outval.index = vertex.index;
    return outval;
}

struct FragmentInput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) index: i32,
}

@group(0) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_array: binding_array<sampler>;

struct Uniforms {
    index: u32,
}

@group(0) @binding(3)
var<uniform> uniforms: Uniforms;

@fragment
fn uniform_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    var outval: vec3<f32>;
    outval = textureSampleLevel(
            texture_array[uniforms.index],
            sampler_array[uniforms.index],
            fragment.tex_coord,
            0.0
        ).rgb;

    return vec4<f32>(outval.x, outval.y, outval.z, 1.0);
}