// shaders02.wgsl
//
// Vertex Shaders - Transforming Positions
//
// The vertex shader's job is to transform vertex positions.
// It receives vertex attributes as input and outputs a position + any varying data.
//
// Input attributes use @location(N) - these come from vertex buffers
// Output position uses @builtin(position) - this is required
// Other outputs use @location(N) - these go to the fragment shader
//
// Your task: Complete the vertex shader to pass color to the fragment shader!

// I AM NOT DONE

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,  // This will be interpolated across the triangle
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // TODO: Set the clip_position from the input position
    // Remember: clip_position is vec4, input.position is vec2
    // Set z=0.0 and w=1.0
    output.clip_position = vec4<f32>(input.position, ____, ____);  // FIX ME!

    // TODO: Pass through the color
    output.color = ____;  // FIX ME!

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Use the interpolated color from the vertex shader
    return vec4<f32>(input.color, 1.0);
}
