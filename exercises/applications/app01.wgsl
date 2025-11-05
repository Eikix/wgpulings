// app01.wgsl
// Compute shader for app01

// I AM NOT DONE

struct Pixel {
                r: u32,
                g: u32,
                b: u32,
                a: u32,
            }

            @group(0) @binding(0) var<storage, read> input: array<Pixel>;
            @group(0) @binding(1) var<storage, read_write> output: array<Pixel>;

            @compute @workgroup_size(8, 8)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let index = gid.y * 4u + gid.x;  // 4 = width
                if (index < arrayLength(&input)) {
                    let pixel = input[index];

                    // TODO: Calculate grayscale value
                    // Formula: gray = 0.299*r + 0.587*g + 0.114*b
                    let gray = u32(
                        f32(pixel.r) * ____ +  // FIX ME!
                        f32(pixel.g) * ____ +  // FIX ME!
                        f32(pixel.b) * ____    // FIX ME!
                    );

                    output[index] = Pixel(gray, gray, gray, pixel.a);
                }
            }
