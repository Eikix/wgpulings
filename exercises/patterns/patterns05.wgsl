// patterns05.wgsl
// Compute shader for patterns05

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> values: array<u32>;
            @group(0) @binding(1) var<storage, read_write> histogram: array<atomic<u32>>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&values)) {
                    let value = values[i];
                    // TODO: Atomically increment histogram[value]
                    // Use: atomicAdd(&histogram[value], 1u);
                    ____(&histogram[____], ____);  // FIX ME!
                }
            }
