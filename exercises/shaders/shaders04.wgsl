// shaders04.wgsl
//
// Uniform Buffers - Passing Data to Shaders
//
// Uniforms are values that stay constant for an entire draw call.
// Common uses:
// - Transformation matrices
// - Time (for animations)
// - Light positions
// - Material properties
//
// Syntax:
// @group(0) @binding(0) var<uniform> name: Type;
//
// Your task: Use a uniform to control color!

// I AM NOT DONE

// Define a uniform buffer structure
struct Uniforms {
    color: vec4<f32>,
    time: f32,
}

// Declare the uniform
// @group and @binding numbers must match your Rust code
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // TODO: Return the color from the uniform buffer
    return ____;  // FIX ME! Use uniforms.color
}
