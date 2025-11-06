// perf03.rs
//
// Bank Conflicts in Shared Memory
//
// Shared memory is divided into banks (typically 32).
// If multiple threads access the same bank simultaneously = conflict = serialization!
//
// GOOD: Thread i accesses shared[i] (different banks)
// BAD: Thread i accesses shared[i * 2] on 32-bank memory (conflicts!)
//
// Your task: Understand bank conflict patterns!
//
// Execute `wgpulings hint perf03` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    println!("Bank conflicts demo");
    println!("Note: This is educational - actual conflicts are GPU-specific\n");

    // TODO: This shader demonstrates potential bank conflicts
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Bank Conflict Example"),
        source: wgpu::ShaderSource::Wgsl(include_str!("perf03.wgsl").into()),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &shader,
        entry_point: "main",
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.dispatch_workgroups(1, 1, 1);
    }
    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);

    println!("✓ Shader executed");
    println!("\n🎉 Bank conflicts understanding!");
    println!("\nTo avoid conflicts:");
    println!("  - Use sequential access when possible");
    println!("  - Pad shared memory arrays");
    println!("  - Use profiling tools to detect conflicts");
}
