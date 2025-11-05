// patterns01.wgsl
// Compute shader for patterns01

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> celsius: array<f32>;
            @group(0) @binding(1) var<storage, read_write> fahrenheit: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&celsius)) {
                    // TODO: Convert C to F
                    // Formula: F = C * 1.8 + 32
                    fahrenheit[i] = celsius[i] * ____ + ____;  // FIX ME!
                }
            }
