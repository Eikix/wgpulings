// app05.rs - N-Body Simulation
// Each body affects all others: O(n²)
// Perfect for GPU parallelism!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        source: wgpu::ShaderSource::Wgsl(r#"
            struct Body {
                pos: vec2<f32>,
                vel: vec2<f32>,
                mass: f32,
            }

            @group(0) @binding(0) var<storage, read> bodies_in: array<Body>;
            @group(0) @binding(1) var<storage, read_write> bodies_out: array<Body>;

            const G: f32 = 0.1;  // Gravitational constant

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i >= arrayLength(&bodies_in)) { return; }

                var body = bodies_in[i];
                var force = vec2<f32>(0.0, 0.0);

                // Sum forces from all other bodies
                for (var j = 0u; j < arrayLength(&bodies_in); j++) {
                    if (i == j) { continue; }

                    let other = bodies_in[j];
                    let delta = other.pos - body.pos;
                    let dist_sq = dot(delta, delta) + ____;  // FIX ME! Add 0.01 to avoid singularity
                    let dist = sqrt(dist_sq);

                    // F = G * m1 * m2 / r²
                    force += normalize(delta) * (G * body.mass * other.mass / dist_sq);
                }

                // Update velocity and position
                body.vel += force / body.mass * 0.01;  // dt = 0.01
                body.pos += body.vel * 0.01;

                bodies_out[i] = body;
            }
        "#.into()),
        label: None,
    });

    println!("🎉 N-body simulation shader created!");
}
