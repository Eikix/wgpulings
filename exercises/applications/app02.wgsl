// app02.wgsl
// Compute shader for app02

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;

            const WIDTH: u32 = 8u;
            const HEIGHT: u32 = 8u;

            fn get_pixel(x: i32, y: i32) -> f32 {
                if (x < 0 || x >= i32(WIDTH) || y < 0 || y >= i32(HEIGHT)) {
                    return 0.0;  // Clamp to edge
                }
                return input[u32(y) * WIDTH + u32(x)];
            }

            @compute @workgroup_size(8, 8)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let x = i32(gid.x);
                let y = i32(gid.y);

                if (gid.x >= WIDTH || gid.y >= HEIGHT) {
                    return;
                }

                // TODO: Average 3x3 neighborhood
                var sum = 0.0;
                for (var dy = -1; dy <= 1; dy++) {
                    for (var dx = -1; dx <= 1; dx++) {
                        sum += get_pixel(x + dx, y + dy);
                    }
                }

                let index = gid.y * WIDTH + gid.x;
                output[index] = sum / ____;  // FIX ME! How many pixels in 3x3?
            }
