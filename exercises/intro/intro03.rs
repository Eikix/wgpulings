// intro03.rs
//
// Buffers: Moving Data Between CPU and GPU
//
// The GPU has its own memory (VRAM), separate from CPU RAM.
// To do useful work, you need to:
// 1. Upload data from CPU → GPU (input)
// 2. Run compute shader on GPU
// 3. Download results from GPU → CPU (output)
//
// Buffers are chunks of GPU memory that hold your data.
//
// Your task: Create buffers, upload data, run a compute shader, and read results back!
//
// Execute `wgpulings hint intro03` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    // Our input data: Just some numbers
    let numbers = vec![1.0f32, 2.0, 3.0, 4.0, 5.0];
    println!("Input: {:?}", numbers);

    // STEP 1: Create a STORAGE buffer with our data
    // Storage buffers can be read/written by compute shaders
    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        contents: bytemuck::cast_slice(&numbers),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    println!("✓ Created input buffer ({} bytes)", numbers.len() * 4);

    // STEP 2: Create an OUTPUT buffer (initially empty)
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: (numbers.len() * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // STEP 3: Create a STAGING buffer to read results back to CPU
    // MAP_READ means we can map this buffer to CPU memory
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: (numbers.len() * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    println!("✓ Created output and staging buffers");

    // TODO: Create a compute shader that doubles each number
    // input[i] * 2 → output[i]
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Doubler Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read> input: array<f32>;
            @group(0) @binding(1) var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                if (index < arrayLength(&input)) {
                    // TODO: Write code to double the input value
                    output[index] = input[index] * ____;  // FIX ME!
                }
            }
        "#.into()),
    });

    // Create bind group layout and bind group
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
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
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
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
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    // Run the compute shader!
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(1, 1, 1);  // 1 workgroup is enough for 5 elements
    }

    // Copy output to staging buffer so we can read it
    encoder.copy_buffer_to_buffer(
        &output_buffer,
        0,
        &staging_buffer,
        0,
        (numbers.len() * std::mem::size_of::<f32>()) as u64,
    );

    queue.submit(Some(encoder.finish()));

    // Map the staging buffer to read results
    let buffer_slice = staging_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buffer.unmap();

    println!("\n✓ Compute complete!");
    println!("Output: {:?}", result);
    println!("\n🎉 Success! You moved data to GPU, processed it, and got results back!");
}
