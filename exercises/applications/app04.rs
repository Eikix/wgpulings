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
        source: wgpu::ShaderSource::Wgsl(include_str!("app04.wgsl").into()),
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
