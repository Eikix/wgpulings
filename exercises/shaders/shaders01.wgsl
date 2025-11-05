// shaders01.wgsl
//
// WGSL Basics - Syntax and Structure
//
// WGSL (WebGPU Shading Language) is the shader language for WebGPU.
// It's similar to Rust in some ways (explicit types, modern syntax).
//
// Key concepts:
// - Variables: var name: type = value;
// - Constants: const name: type = value;
// - Functions: fn name(params) -> return_type { }
// - Types: f32 (float), i32 (int), u32 (unsigned), bool
// - Vectors: vec2<f32>, vec3<f32>, vec4<f32>
// - Matrices: mat2x2<f32>, mat3x3<f32>, mat4x4<f32>
//
// Your task: Fix the syntax errors in this shader!

// I AM NOT DONE

// Vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // TODO: Define an array of 3 positions for a triangle
    // Each position is a vec2<f32>
    // Syntax: var name = array<type, size>(element1, element2, ...);
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(____, ____)  // FIX ME! Complete the third vertex
    );

    // Return a vec4 (x, y, z, w) for clip space position
    // The last component (w) is usually 1.0
    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

// Fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    // TODO: Return a color (RGBA)
    // Let's make it purple: red=0.5, green=0.0, blue=1.0, alpha=1.0
    return vec4<f32>(____, ____, ____, ____);  // FIX ME!
}
