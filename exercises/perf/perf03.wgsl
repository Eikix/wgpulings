// perf03.wgsl
// Compute shader for perf03

// I AM NOT DONE

var<workgroup> shared: array<f32, 256>;

            @compute @workgroup_size(256)
            fn main(
                @builtin(local_invocation_id) lid: vec3<u32>,
                @builtin(global_invocation_id) gid: vec3<u32>
            ) {
                let local_id = lid.x;

                // NO CONFLICT: Each thread writes to different location
                shared[local_id] = f32(local_id);
                workgroupBarrier();

                // TODO: Try accessing with stride (potential conflict)
                // On 32-bank memory, stride of 32 causes conflicts!
                let stride_access = shared[(local_id * ____) % 256u];  // FIX ME! Try 32 or 1

                // Use the value (prevent optimization)
                if (stride_access > 1000.0) {
                    shared[0] = 1.0;
                }
            }
