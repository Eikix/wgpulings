// app04.rs - Particle Simulation
// Update many particles in parallel!
// Each thread = one particle

// I AM NOT DONE

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Particle {
    pos: [f32; 2],
    vel: [f32; 2],
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let particles = vec![
        Particle { pos: [0.0, 0.0], vel: [1.0, 1.0] },
        Particle { pos: [0.5, 0.0], vel: [-1.0, 1.0] },
    ];

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: bytemuck::cast_slice(&particles),
        usage: wgpu::BufferUsages::STORAGE,
        label: None,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        source: wgpu::ShaderSource::Wgsl(r#"
            struct Particle {
                pos: vec2<f32>,
                vel: vec2<f32>,
            }

            @group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&particles)) {
                    var p = particles[i];

                    // Update position
                    p.pos = p.pos + p.vel * ____;  // FIX ME! dt = 0.01

                    // Bounce off walls
                    if (abs(p.pos.x) > 1.0) { p.vel.x = -p.vel.x; }
                    if (abs(p.pos.y) > 1.0) { p.vel.y = -p.vel.y; }

                    particles[i] = p;
                }
            }
        "#.into()),
        label: None,
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        layout: None,
        module: &shader,
        entry_point: "main",
        label: None,
    });

    println!("🎉 Particle simulation ready!");
}
