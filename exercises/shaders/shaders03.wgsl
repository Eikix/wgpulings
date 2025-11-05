// shaders03.wgsl
//
// Fragment Shaders - Determining Pixel Colors
//
// The fragment shader runs once per pixel and determines its final color.
// It receives interpolated data from the vertex shader.
//
// Common tasks:
// - Apply textures
// - Calculate lighting
// - Mix colors
// - Apply effects
//
// Your task: Create a gradient effect using the UV coordinates!

// I AM NOT DONE

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,  // UV coordinates (0 to 1)
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0)
    );

    // Convert position from -1..1 to UV 0..1
    var uvs = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 0.0)
    );

    var output: VertexOutput;
    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
    output.uv = uvs[vertex_index];
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // TODO: Create a gradient using UV coordinates
    // UV.x goes from 0 (left) to 1 (right)
    // UV.y goes from 0 (top) to 1 (bottom)
    //
    // Make a gradient that:
    // - Interpolates from red (left) to blue (right)
    // - Red channel = 1.0 - uv.x (full red on left, none on right)
    // - Blue channel = uv.x (none on left, full blue on right)

    var red = ____;    // FIX ME!
    var green = 0.0;
    var blue = ____;   // FIX ME!

    return vec4<f32>(red, green, blue, 1.0);
}
