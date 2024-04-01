@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

var<push_constant> camera_projection: mat4x4<f32>;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) xy: u32,
    @location(1) z: f32,
    @location(2) color: vec4<f32>,
    @location(3) source: vec2<u32>,
    @location(4) index: i32,
}

fn map_source(source: vec4<f32>, texture_dimensions: vec2<u32>) -> vec4<f32> {
    if source.w == 0.0 && source.z == 0.0 {
        return vec4<f32>(0.0, 0.0, f32(texture_dimensions.x), f32(texture_dimensions.y));
    }

    return source;
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    var position: vec2<f32>;

    let texture_dimensions: vec2<u32> = textureDimensions(texture);

    let unpacked_source = vec4<f32>(f32(input.source.x & 0xFFFFu), f32(input.source.x >> 16u), f32(input.source.y & 0xFFFFu), f32(input.source.y >> 16u));

    let source: vec4<f32> = map_source(unpacked_source, texture_dimensions);

    let y = f32(input.xy >> 16u);
    let x = f32(input.xy & 0xFFFFu);

    switch (input.vertex_index) {
        case 0u: {
            let left = x;
            let top = y;
            position = vec2<f32>(left, top);

            let tex_left = source.x / f32(texture_dimensions.x);
            let tex_top = (source.y + source.w) / f32(texture_dimensions.y);
            output.texture_position = vec2<f32>(tex_left, tex_top);
            break;
        }
        case 1u: {
            let top = y;
            let right = x + source.z;
            position = vec2<f32>(right, top);

            let tex_top = (source.y + source.w) / f32(texture_dimensions.y);
            let tex_right = (source.x + source.z) / f32(texture_dimensions.x);
            output.texture_position = vec2<f32>(tex_right, tex_top);
            break;
        }
        case 2u: {
            let left = x;
            let bottom = y + source.w;
            position = vec2<f32>(left, bottom);

            let tex_bottom = source.y / f32(texture_dimensions.y);
            let tex_left = source.x / f32(texture_dimensions.x);
            output.texture_position = vec2<f32>(tex_left, tex_bottom);
            break;
        }
        case 3u: {
            let right = x + source.z;
            let bottom = y + source.w;
            position = vec2<f32>(right, bottom);

            let tex_bottom = source.y / f32(texture_dimensions.y);
            let tex_right = (source.x + source.z) / f32(texture_dimensions.x);
            output.texture_position = vec2<f32>(tex_right, tex_bottom);
            break;
        }
        default: {}
    }

    output.position = camera_projection * vec4<f32>(position, 1.0, 1.0);
    output.position.z = input.z;
    output.color = input.color;
    output.index = input.index;

    return output;
}

struct VertexOutput {
    @builtin(position) @invariant position: vec4<f32>,
    @location(0) texture_position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) index: i32,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, texture_sampler, in.texture_position);

    if color.r == 0.0 && color.g == 0.0 && color.b == 0.0 && color.a == 1.0 {
        discard;
    }
    if color.a < 0.0001 {
        discard;
    }

    return color * in.color;
}
