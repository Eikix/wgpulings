// basics03.wgsl
// Compute shader for basics03

// I AM NOT DONE

struct Params {
                scale: f32,
            }

            @group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;
            @group(0) @binding(2) var<uniform> params: Params;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                if (index < arrayLength(&input)) {
                    // TODO: Multiply input by params.scale
                    output[index] = input[index] * ____;  // FIX ME!
                }
            }
