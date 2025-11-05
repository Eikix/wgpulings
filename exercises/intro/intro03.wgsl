// intro03.wgsl
// Compute shader for intro03

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                if (index < arrayLength(&input)) {
                    // TODO: Write code to double the input value
                    output[index] = input[index] * ____;  // FIX ME!
                }
            }
