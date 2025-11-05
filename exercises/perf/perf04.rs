// perf04.rs
//
// Benchmarking GPU Code
//
// To optimize, you must measure!
// Simple approach: Time CPU-side submission
// Better: Use timestamp queries (GPU-side timing)
//
// Your task: Benchmark a compute operation!
//
// Execute `wgpulings hint perf04` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;
use std::time::Instant;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let size = 10_000_000usize;
    let data: Vec<f32> = vec![1.0; size];

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Data"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read_write> data: array<f32>;

            @compute @workgroup_size(256)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&data)) {
                    // Do some work
                    data[i] = sqrt(data[i] * 2.0) + 1.0;
                }
            }
        "#.into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
        label: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
        label: None,
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    // TODO: Benchmark multiple runs
    let num_runs = ____;  // FIX ME! Try 10 runs
    let mut times = Vec::new();

    for _ in 0..num_runs {
        let start = Instant::now();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((size / 256) as u32, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        let elapsed = start.elapsed();
        times.push(elapsed);
    }

    let avg = times.iter().sum::<std::time::Duration>() / num_runs as u32;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();

    println!("Processed {} elements", size);
    println!("Average: {:?}", avg);
    println!("Min: {:?}", min);
    println!("Max: {:?}", max);

    println!("\n🎉 Benchmarking complete!");
    println!("\nTips:");
    println!("  - Run multiple iterations");
    println!("  - Discard first run (warmup)");
    println!("  - Use GPU timestamp queries for precision");
}
