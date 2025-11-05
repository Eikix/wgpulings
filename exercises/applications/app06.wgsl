// app06.wgsl
// Compute shader for app06

// I AM NOT DONE

@group(0) @binding(0) var<storage, read_write> data: array<u32>;

            struct Params {
                stage: u32,
                step: u32,
            }

            @group(0) @binding(1) var<uniform> params: Params;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i >= arrayLength(&data) / 2u) { return; }

                let stage = params.stage;
                let step = params.step;

                // Bitonic sort compare-swap
                let pair_dist = 1u << (stage - step);
                let block_size = 2u * pair_dist;

                let left_id = (i / pair_dist) * block_size + (i % pair_dist);
                let right_id = left_id + pair_dist;

                let ascending = ((left_id >> stage) & 1u) == 0u;

                if (right_id < arrayLength(&data)) {
                    let left_val = data[left_id];
                    let right_val = data[right_id];

                    let swap = (left_val > right_val) == ascending;

                    if (swap) {
                        data[left_id] = ____;  // FIX ME! right_val
                        data[right_id] = ____;  // FIX ME! left_val
                    }
                }
            }
