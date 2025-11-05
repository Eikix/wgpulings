// patterns04.wgsl
// Compute shader for patterns04

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<u32>;
            @group(0) @binding(1) var<storage, read_write> output: array<u32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&input)) {
                    // TODO: Sum all elements up to and including i
                    var sum = 0u;
                    for (var j = 0u; j <= i; j = j + 1u) {
                        sum = sum + ____;  // FIX ME! What to add?
                    }
                    output[i] = sum;
                }
            }
