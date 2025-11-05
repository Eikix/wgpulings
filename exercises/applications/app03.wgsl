// app03.wgsl
// Compute shader for app03

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> a: array<f32>;
            @group(0) @binding(1) var<storage, read> b: array<f32>;
            @group(0) @binding(2) var<storage, read_write> c: array<f32>;

            const N: u32 = 4u;

            @compute @workgroup_size(4, 4)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let row = gid.y;
                let col = gid.x;

                if (row >= N || col >= N) { return; }

                var sum = 0.0;
                for (var k = 0u; k < N; k++) {
                    sum += a[row * N + k] * b[k * N + ____];  // FIX ME!
                }

                c[row * N + col] = sum;
            }
