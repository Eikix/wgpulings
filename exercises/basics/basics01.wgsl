// basics01.wgsl
//
// WGSL Compute Shader Syntax
//
// WGSL (WebGPU Shading Language) is the language for writing GPU programs.
// For compute, the key elements are:
//
// @compute - Marks this as a compute shader
// @workgroup_size(X, Y, Z) - How many threads per workgroup
// @builtin(global_invocation_id) - Unique ID for each thread
// @group/@binding - Which buffers to use
//
// Your task: Complete this compute shader that adds two arrays!
//
// Execute `wgpulings hint basics01` if you need help!

// I AM NOT DONE

// Declare input buffers
// @group(0) @binding(0) means first bind group, first binding
@group(0) @binding(0) var<storage, read> input_a: array<f32>;
@group(0) @binding(1) var<storage, read> input_b: array<f32>;

// TODO: Declare output buffer at binding 2
// It should be read_write, not just read!
@group(0) @binding(2) var<storage, ____> output: array<f32>;  // FIX ME!

// Compute shader entry point
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Get our thread's index
    let index = global_id.x;

    // Check bounds: Don't access beyond array length
    if (index < arrayLength(&input_a)) {
        // TODO: Add input_a[index] + input_b[index] and store in output[index]
        output[index] = ____ + ____;  // FIX ME!
    }
}
