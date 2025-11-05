// intro04.wgsl
// Compute shader for intro04

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<u32>;
            @group(0) @binding(1) var<storage, read_write> output: array<u32>;

            @compute @workgroup_size(256)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;

                // IMPORTANT: Check bounds!
                // We might launch more threads than elements
                if (index < arrayLength(&input)) {
                    output[index] = input[index] * input[index];
                }
            }
