// basics05_multiply.wgsl
// Shader 3: Multiply each value by 2

// I AM NOT DONE

@group(0) @binding(0) var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < arrayLength(&data)) {
        data[i] = data[i] * ____;  // FIX ME!
    }
}
