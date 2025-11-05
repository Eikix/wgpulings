// patterns02.rs
//
// Parallel Reduction - Combining Many Values into One
//
// Reduction combines all elements using an operation (sum, max, min, etc.)
// This is trickier than map because threads need to COOPERATE.
//
// Simple two-stage approach:
// Stage 1: Each workgroup reduces its chunk
// Stage 2: CPU combines workgroup results
//
// Later we'll do full GPU reduction with shared memory!
//
// Your task: Sum all numbers using parallel reduction!
//
// Execute `wgpulings hint patterns02` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Let's sum these numbers: 1+2+3+...+100 = 5050
    let numbers: Vec<u32> = (1..=100).collect();
    let size = numbers.len();
    let workgroup_size = 64;
    let num_workgroups = (size + workgroup_size - 1) / workgroup_size;

    println!("Summing numbers 1..=100");
    println!("Workgroups: {}", num_workgroups);

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input"),
        contents: bytemuck::cast_slice(&numbers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Each workgroup produces one partial sum
    let partial_sums_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Partial Sums"),
        size: (num_workgroups * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (num_workgroups * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Simple reduction: each workgroup sums its elements
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Reduce Sum"),
        source: wgpu::ShaderSource::Wgsl(include_str!("patterns02.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
        label: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: partial_sums_buffer.as_entire_binding(),
            },
        ],
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

    // TODO: Dispatch enough workgroups to cover all elements
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(____ as u32, 1, 1);  // FIX ME! How many workgroups?
    }

    encoder.copy_buffer_to_buffer(&partial_sums_buffer, 0, &staging_buffer, 0, (num_workgroups * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let partial: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

    println!("Partial sums from each workgroup: {:?}", partial);

    // Final reduction on CPU
    let total: u32 = partial.iter().sum();

    println!("\nTotal sum: {}", total);
    println!("Expected: 5050");

    if total == 5050 {
        println!("\n✓ Correct!");
        println!("🎉 Success! You've done a two-stage reduction!");
        println!("\nNext: We'll do a full GPU reduction with shared memory!");
    } else {
        println!("\n❌ Sum is incorrect!");
    }
}
