// patterns02.wgsl
// Compute shader for patterns02

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<u32>;
            @group(0) @binding(1) var<storage, read_write> partial_sums: array<u32>;

            @compute @workgroup_size(64)
            fn main(
                @builtin(global_invocation_id) gid: vec3<u32>,
                @builtin(local_invocation_id) lid: vec3<u32>,
                @builtin(workgroup_id) wid: vec3<u32>
            ) {
                let global_index = gid.x;
                let local_index = lid.x;
                let workgroup_index = wid.x;

                // Each thread sums its assigned elements
                var sum = 0u;
                let stride = 64u;  // workgroup size

                // Sum elements for this thread
                for (var i = global_index; i < arrayLength(&input); i = i + stride) {
                    sum = sum + input[i];
                }

                // TODO: Store the sum
                // For now, just write to output (not optimal, but simple!)
                // We'll use atomics in the next exercise
                if (local_index == 0u) {
                    partial_sums[workgroup_index] = sum;
                }
            }
