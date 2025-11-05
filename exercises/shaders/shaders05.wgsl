// shaders05.wgsl
//
// Varying Data - Communication Between Shader Stages
//
// Data flows through the pipeline:
// 1. Vertex shader outputs data with @location(N)
// 2. GPU interpolates that data across the triangle
// 3. Fragment shader receives interpolated data at @location(N)
//
// The @location numbers MUST match between stages!
//
// Your task: Pass multiple pieces of data from vertex to fragment shader!

// I AM NOT DONE

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Set position
    output.position = vec4<f32>(input.position, 0.0, 1.0);

    // TODO: Pass through color and UV
    output.color = ____;  // FIX ME!
    output.uv = ____;     // FIX ME!

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // TODO: Mix the vertex color with a UV-based gradient
    // Use the UV to create a gradient that modulates the color

    // Create a gradient from UV
    var gradient = input.uv.x;  // Left (0.0) to right (1.0)

    // Mix the vertex color with white based on the gradient
    // mix(a, b, t) = a * (1-t) + b * t
    var final_color = mix(input.color, vec3<f32>(1.0, 1.0, 1.0), ____);  // FIX ME! Use gradient

    return vec4<f32>(final_color, 1.0);
}
