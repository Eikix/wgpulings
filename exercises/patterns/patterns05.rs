// patterns05.rs
//
// Histogram - Counting Occurrences
//
// A histogram counts how many values fall into each bin.
// Example: Count pixel brightness (0-255 → 256 bins)
//
// Challenge: Multiple threads may update the same bin!
// Solution: Use atomic operations for thread-safe increments.
//
// Your task: Build a histogram using atomics!
//
// Execute `wgpulings hint patterns05` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Generate random-ish values 0-9
    let values: Vec<u32> = vec![
        1, 5, 3, 7, 2, 8, 1, 4, 5, 9,
        2, 6, 3, 7, 1, 8, 4, 5, 2, 6,
    ];
    let num_bins = 10u32;

    println!("Values: {:?}", values);
    println!("Counting occurrences in {} bins (0-9)", num_bins);

    let values_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Values"),
        contents: bytemuck::cast_slice(&values),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Histogram buffer - initialized to zeros
    let histogram_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Histogram"),
        contents: bytemuck::cast_slice(&vec![0u32; num_bins as usize]),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (num_bins * 4) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Histogram"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read> values: array<u32>;
            @group(0) @binding(1) var<storage, read_write> histogram: array<atomic<u32>>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&values)) {
                    let value = values[i];
                    // TODO: Atomically increment histogram[value]
                    // Use: atomicAdd(&histogram[value], 1u);
                    ____(&histogram[____], ____);  // FIX ME!
                }
            }
        "#.into()),
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
                resource: values_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: histogram_buffer.as_entire_binding(),
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

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&histogram_buffer, 0, &staging_buffer, 0, (num_bins * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let histogram: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

    println!("\nHistogram:");
    for (bin, count) in histogram.iter().enumerate() {
        println!("  Bin {}: {}", bin, count);
    }

    let total: u32 = histogram.iter().sum();
    if total == values.len() as u32 {
        println!("\n✓ Total counts match!");
        println!("🎉 Success! You've built a histogram with atomics!");
    } else {
        println!("\n❌ Total is {} but expected {}", total, values.len());
    }
}
