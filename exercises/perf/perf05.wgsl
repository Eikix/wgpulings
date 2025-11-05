// perf05.wgsl
// Compute shader for perf05

// I AM NOT DONE

// Large shared memory limits how many workgroups can run
            var<workgroup> big_shared: array<f32, 2048>;

            @compute @workgroup_size(256)
            fn main(@builtin(local_invocation_id) lid: vec3<u32>) {
                big_shared[lid.x] = f32(lid.x);
                workgroupBarrier();
                let value = big_shared[(lid.x + 1u) % 256u];
                if (value > 1000.0) {
                    big_shared[0] = 1.0;
                }
            }
