// perf04.wgsl
// Compute shader for perf04

// I AM NOT DONE

@group(0) @binding(0) var<storage, read_write> data: array<f32>;

            @compute @workgroup_size(256)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&data)) {
                    // Do some work
                    data[i] = sqrt(data[i] * 2.0) + 1.0;
                }
            }
