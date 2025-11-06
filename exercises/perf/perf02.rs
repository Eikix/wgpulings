// perf02.rs
//
// Memory Coalescing - Access Patterns Matter!
//
// GPUs load memory in chunks (32, 64, or 128 bytes).
// Adjacent threads accessing adjacent memory = ONE load (coalesced)
// Scattered accesses = MANY loads (uncoalesced) = SLOW!
//
// GOOD: thread i accesses element i
// BAD: thread i accesses element (i * stride) with large stride
//
// Your task: Compare coalesced vs. uncoalesced access!
//
// Execute `wgpulings hint perf02` if you need help!

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

    let size = 1_000_000usize;
    let data: Vec<f32> = vec![1.0; size];

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input"),
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output"),
        size: (size * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    println!("Comparing memory access patterns\n");

    // COALESCED: Adjacent threads → adjacent memory
    let shader_coalesced = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Coalesced"),
        source: wgpu::ShaderSource::Wgsl(include_str!("perf02.wgsl").into()),
    });

    // TODO: Create UNCOALESCED shader
    // Thread i should access element (i * 7) % array_length
    // This creates a scattered access pattern
    let shader_uncoalesced = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Uncoalesced"),
        source: wgpu::ShaderSource::Wgsl(include_str!("perf02.wgsl").into()),
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
                resource: output_buffer.as_entire_binding(),
            },
        ],
        label: None,
    });

    for (name, shader) in &[
        ("Coalesced", &shader_coalesced),
        ("Uncoalesced", &shader_uncoalesced),
    ] {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
            label: None,
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(name),
            layout: Some(&pipeline_layout),
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
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups((size / 256) as u32, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        println!("{}: Done", name);
    }

    println!("\n🎉 Both patterns completed!");
    println!("\nCoalesced access is typically MUCH faster!");
    println!("On some GPUs, uncoalesced can be 10x slower!");
}
