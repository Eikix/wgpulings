// intro02.wgsl
//
// Your First Compute Shader!
//
// @compute marks this as a compute shader (not vertex/fragment)
// @workgroup_size(64) means 64 threads run together per workgroup
//
// If we dispatch 10 workgroups, we get 64 * 10 = 640 threads running in parallel!

// I AM NOT DONE

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // global_id.x is the unique thread ID
    // Thread 0, 1, 2, ... 639 all run in parallel

    // For now, this shader does nothing - just demonstrates structure
    // Later exercises will do real work here!
}
