// perf05.rs
//
// Occupancy - How Many Threads Can Run Simultaneously?
//
// Occupancy = Active warps / Maximum possible warps
//
// Limited by:
// - Registers per thread
// - Shared memory per workgroup
// - Workgroup size
//
// Higher occupancy → Better latency hiding → Better performance (usually!)
//
// Your task: Understand occupancy trade-offs!
//
// Execute `wgpulings hint perf05` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    println!("Occupancy demonstration\n");

    // LOW OCCUPANCY: Large shared memory usage
    let shader_low_occ = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Low Occupancy"),
        source: wgpu::ShaderSource::Wgsl(r#"
            // Large shared memory limits how many workgroups can run
            var<workgroup> big_shared: array<f32, 2048>;

            @compute @workgroup_size(256)
            fn main(@builtin(local_invocation_id) lid: vec3<u32>) {
                big_shared[lid.x] = f32(lid.x);
                workgroupBarrier();
                let value = big_shared[(lid.x + 1u) % 256u];
                if (value > 1000.0) {
                    big_shared[0] = 1.0;
                }
            }
        "#.into()),
    });

    // TODO: HIGH OCCUPANCY: Minimal resources
    let shader_high_occ = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("High Occupancy"),
        source: wgpu::ShaderSource::Wgsl(r#"
            // Small shared memory allows more concurrent workgroups
            var<workgroup> small_shared: array<f32, ____>;  // FIX ME! Use 64

            @compute @workgroup_size(64)
            fn main(@builtin(local_invocation_id) lid: vec3<u32>) {
                small_shared[lid.x] = f32(lid.x);
                workgroupBarrier();
                let value = small_shared[(lid.x + 1u) % 64u];
                if (value > 1000.0) {
                    small_shared[0] = 1.0;
                }
            }
        "#.into()),
    });

    for (name, shader) in &[("Low Occupancy", &shader_low_occ), ("High Occupancy", &shader_high_occ)] {
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(name),
            layout: None,
            module: shader,
            entry_point: "main",
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.dispatch_workgroups(100, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        println!("{}: ✓", name);
    }

    println!("\n🎉 Occupancy examples complete!");
    println!("\nKey points:");
    println!("  - More resources per thread = Lower occupancy");
    println!("  - Lower occupancy = Fewer threads to hide latency");
    println!("  - Balance: Enough resources vs. enough threads");
    println!("  - Profile to find optimal point!");
}
