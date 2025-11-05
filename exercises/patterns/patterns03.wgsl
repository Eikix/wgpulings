// patterns03.wgsl
//
// Workgroup Shared Memory - Fast Cooperation Between Threads
//
// Shared memory is MUCH faster than global memory!
// It's shared by all threads in a workgroup.
//
// Use it for:
// - Reductions (sum, max, min)
// - Prefix scans
// - Tiling in matrix multiply
// - Any algorithm where threads cooperate
//
// Key points:
// - Declared with var<workgroup>
// - Size must be compile-time constant
// - Need workgroupBarrier() to synchronize
// - Only visible within one workgroup
//
// Your task: Use shared memory for a workgroup reduction!
//
// Execute `wgpulings hint patterns03` if you need help!

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

// TODO: Declare shared memory array
// Size should match workgroup size (256)
var<workgroup> shared_data: array<f32, ____>;  // FIX ME!

@compute @workgroup_size(256)
fn main(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>
) {
    let global_idx = gid.x;
    let local_idx = lid.x;
    let workgroup_idx = wid.x;

    // Step 1: Load data into shared memory
    if (global_idx < arrayLength(&input)) {
        shared_data[local_idx] = input[global_idx];
    } else {
        shared_data[local_idx] = 0.0;
    }

    // TODO: Wait for all threads to finish loading
    ____();  // FIX ME! What barrier function?

    // Step 2: Tree reduction in shared memory
    // This is O(log N) parallel reduction!
    var stride = 128u;  // Half of workgroup size
    while (stride > 0u) {
        if (local_idx < stride) {
            shared_data[local_idx] = shared_data[local_idx] + shared_data[local_idx + stride];
        }

        // TODO: Barrier after each reduction step
        ____();  // FIX ME!

        stride = stride / 2u;
    }

    // Step 3: Thread 0 writes the result
    if (local_idx == 0u) {
        output[workgroup_idx] = shared_data[0];
    }
}
