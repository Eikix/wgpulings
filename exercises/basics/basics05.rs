// basics05.rs
//
// Multiple Compute Passes - Chaining Operations
//
// You can run multiple compute operations in one command buffer!
// This avoids CPU-GPU round trips and is MUCH faster.
//
// Example pipeline:
// 1. Pass 1: Square all numbers
// 2. Pass 2: Add 10 to all numbers
// 3. Pass 3: Multiply by 2
//
// All in one submission!
//
// Your task: Chain multiple compute passes together!
//
// Execute `wgpulings hint basics05` if you need help!

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

    let input: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let size = input.len();

    println!("Input: {:?}", input);
    println!("Pipeline: square → add 10 → multiply by 2");

    let data_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Data"),
        contents: bytemuck::cast_slice(&input),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (size * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Shader 1: Square each value
    let shader1 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Square"),
        source: wgpu::ShaderSource::Wgsl(include_str!("basics05.wgsl").into()),
    });

    // Shader 2: Add 10
    let shader2 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Add 10"),
        source: wgpu::ShaderSource::Wgsl(include_str!("basics05.wgsl").into()),
    });

    // TODO: Create shader 3 that multiplies by 2
    let shader3 = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Multiply by 2"),
        source: wgpu::ShaderSource::Wgsl(include_str!("basics05.wgsl").into()),
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
            resource: data_buffer.as_entire_binding(),
        }],
        label: None,
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
        label: None,
    });

    let pipeline1 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Pipeline 1"),
        layout: Some(&pipeline_layout),
        module: &shader1,
        entry_point: "main",
    });

    let pipeline2 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Pipeline 2"),
        layout: Some(&pipeline_layout),
        module: &shader2,
        entry_point: "main",
    });

    let pipeline3 = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Pipeline 3"),
        layout: Some(&pipeline_layout),
        module: &shader3,
        entry_point: "main",
    });

    // TODO: Run all three pipelines in ONE command buffer!
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });

        // Pass 1: Square
        pass.set_pipeline(&pipeline1);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);

        // TODO: Pass 2: Add 10
        pass.set_pipeline(&____); // FIX ME!
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);

        // TODO: Pass 3: Multiply by 2
        pass.set_pipeline(&____); // FIX ME!
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&data_buffer, 0, &staging_buffer, 0, (size * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

    println!("\nOutput: {:?}", result);
    println!("\nExpected: [2, 28, 38, 52, 70]");
    println!("  (1²+10)*2=22, (2²+10)*2=28, (3²+10)*2=38, etc.");

    let expected = vec![22.0, 28.0, 38.0, 52.0, 70.0];
    let correct = result
        .iter()
        .zip(&expected)
        .all(|(a, b)| (a - b).abs() < 0.01);

    if correct {
        println!("\n✓ All passes executed correctly!");
        println!("🎉 Success! You chained multiple compute operations!");
    } else {
        println!("\n❌ Results don't match!");
    }
}
