// basics04.rs
//
// Reading Results Back to CPU
//
// GPU compute results need to be copied back to CPU memory to be useful!
//
// The process:
// 1. Compute shader writes to storage buffer (on GPU)
// 2. Copy storage buffer → staging buffer (still on GPU)
// 3. Map staging buffer to CPU memory
// 4. Read the data from CPU!
//
// Your task: Complete the buffer copy and mapping code!
//
// Execute `wgpulings hint basics04` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Create some data: squares of 0..9
    let size = 10usize;
    let input: Vec<u32> = (0..size as u32).collect();

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input"),
        contents: bytemuck::cast_slice(&input),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output"),
        size: (size * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // TODO: Create staging buffer with MAP_READ and COPY_DST usage
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (size * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::____ | wgpu::BufferUsages::____,  // FIX ME!
        mapped_at_creation: false,
    });

    // Shader that squares numbers
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Square Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read> input: array<u32>;
            @group(0) @binding(1) var<storage, read_write> output: array<u32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&input)) {
                    output[i] = input[i] * input[i];
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
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
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

    // TODO: Copy output_buffer to staging_buffer
    // Hint: encoder.copy_buffer_to_buffer(src, src_offset, dst, dst_offset, size)
    encoder.copy_buffer_to_buffer(
        &____,  // FIX ME! Source buffer
        0,
        &____,  // FIX ME! Destination buffer
        0,
        (size * std::mem::size_of::<u32>()) as u64,
    );

    queue.submit(Some(encoder.finish()));

    // TODO: Map the staging buffer and read data
    let buffer_slice = staging_buffer.slice(..);

    // Request mapping
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});

    // Wait for GPU to finish
    device.poll(wgpu::Maintain::Wait);

    // Get mapped range
    let data = buffer_slice.get_mapped_range();
    let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);

    // Unmap so we can use the buffer again
    staging_buffer.unmap();

    println!("Input:  {:?}", input);
    println!("Output: {:?}", result);

    // Verify
    let correct = result.iter().enumerate().all(|(i, &val)| val == (i as u32) * (i as u32));
    if correct {
        println!("\n✓ All values correct!");
        println!("🎉 Success! You read GPU results back to CPU!");
    } else {
        println!("\n❌ Something went wrong!");
    }
}
