// intro04.rs
//
// Understanding Workgroups and Dispatch
//
// GPU parallelism has a hierarchy:
// - Dispatch: How many workgroups to launch (X, Y, Z)
// - Workgroup: Group of threads defined in shader @workgroup_size(X, Y, Z)
// - Thread: Individual invocation with unique global_invocation_id
//
// Example: Process 1000 elements with workgroup size 256
// - Dispatch: 4 workgroups (ceil(1000 / 256))
// - Each workgroup: 256 threads
// - Total threads: 1024 (some do nothing if index >= 1000)
//
// Your task: Calculate the correct dispatch size for your data!
//
// Execute `wgpulings hint intro04` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Let's process a bigger array this time!
    let size = 1000;
    let workgroup_size = 256;  // Defined in shader

    // Create array: [0, 1, 2, ..., 999]
    let numbers: Vec<u32> = (0..size).collect();

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input"),
        contents: bytemuck::cast_slice(&numbers),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output"),
        size: (size * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (size * std::mem::size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Compute shader that squares each number
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read> input: array<u32>;
            @group(0) @binding(1) var<storage, read_write> output: array<u32>;

            @compute @workgroup_size(256)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;

                // IMPORTANT: Check bounds!
                // We might launch more threads than elements
                if (index < arrayLength(&input)) {
                    output[index] = input[index] * input[index];
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

    // TODO: Calculate how many workgroups we need
    // Formula: ceil(num_elements / workgroup_size)
    //
    // For 1000 elements with workgroup_size 256:
    // ceil(1000 / 256) = ceil(3.906) = 4 workgroups
    //
    // In Rust: (size + workgroup_size - 1) / workgroup_size

    let num_workgroups = (____ + ____ - 1) / ____;  // FIX ME!

    println!("Processing {} elements", size);
    println!("Workgroup size: {}", workgroup_size);
    println!("Dispatching {} workgroups", num_workgroups);
    println!("Total threads: {}", num_workgroups * workgroup_size);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);

        // TODO: Dispatch the correct number of workgroups!
        pass.dispatch_workgroups(____, 1, 1);  // FIX ME!
    }

    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, (size * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

    // Verify: Check first and last few elements
    println!("\n✓ Compute complete!");
    println!("First few: {:?}", &result[0..5]);
    println!("Last few: {:?}", &result[995..1000]);

    // Verify correctness
    let correct = result.iter().enumerate().all(|(i, &val)| val == (i as u32) * (i as u32));
    if correct {
        println!("\n🎉 Perfect! All {} elements computed correctly!", size);
        println!("You now understand workgroups and dispatching!");
    } else {
        println!("\n❌ Something's wrong - check your calculations!");
    }
}
