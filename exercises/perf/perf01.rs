// perf01.rs
//
// Workgroup Size Optimization
//
// Workgroup size affects performance significantly!
//
// Too small (1, 4, 8):
// - Underutilizes GPU
// - More dispatch overhead
//
// Too large (512, 1024):
// - May exceed hardware limits
// - Reduces flexibility for scheduler
// - May limit occupancy
//
// Sweet spots: 64, 128, 256
//
// Your task: Test different workgroup sizes and see the pattern!
//
// Execute `wgpulings hint perf01` if you need help!

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

    let size = 1_000_000;
    let data: Vec<f32> = (0..size).map(|i| i as f32).collect();

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Data"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE,
    });

    println!("Testing workgroup sizes with {} elements\n", size);

    // TODO: Test different workgroup sizes
    let workgroup_sizes = vec![____, ____, ____, ____];  // FIX ME! Try: 32, 64, 128, 256

    for &wg_size in &workgroup_sizes {
        let shader_source = format!(r#"
            @group(0) @binding(0) var<storage, read_write> data: array<f32>;

            @compute @workgroup_size({})
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {{
                let i = gid.x;
                if (i < arrayLength(&data)) {{
                    // Do some work
                    data[i] = sqrt(data[i]) * 2.0 + 1.0;
                }}
            }}
        "#, wg_size);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
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

        let num_workgroups = (size + wg_size - 1) / wg_size;

        let start = Instant::now();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(num_workgroups as u32, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        let elapsed = start.elapsed();

        println!("Workgroup size {:3}: {:?} ({} workgroups)",
                 wg_size, elapsed, num_workgroups);
    }

    println!("\n🎉 Experiment complete!");
    println!("\nNote: Performance varies by GPU architecture.");
    println!("Multiple of 32 (warp size) is usually good!");
}
