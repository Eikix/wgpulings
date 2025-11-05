// patterns01.rs
//
// Parallel Map - The Simplest Pattern
//
// Map applies a function to every element independently.
// This is "embarrassingly parallel" - perfect for GPUs!
//
// Pattern:
//   for each element i:
//     output[i] = f(input[i])
//
// No dependencies between elements = perfect parallelism!
//
// Examples:
// - Convert temperatures F → C
// - Square root of every number
// - Image brightness adjustment
//
// Your task: Implement a parallel map that converts Celsius to Fahrenheit!
//
// Execute `wgpulings hint patterns01` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // Temperatures in Celsius
    let celsius: Vec<f32> = vec![0.0, 10.0, 20.0, 30.0, 37.0, 100.0];
    let size = celsius.len();

    println!("Temperatures in Celsius: {:?}", celsius);

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Celsius"),
        contents: bytemuck::cast_slice(&celsius),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Fahrenheit"),
        size: (size * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (size * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // TODO: Complete the shader to convert C to F
    // Formula: F = C * 9/5 + 32
    // Or: F = C * 1.8 + 32
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("C to F"),
        source: wgpu::ShaderSource::Wgsl(include_str!("patterns01.wgsl").into()),
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

    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, (size * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let fahrenheit_result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

    println!("Temperatures in Fahrenheit: {:?}", fahrenheit_result);
    println!("\nExpected: [32.0, 50.0, 68.0, 86.0, 98.6, 212.0]");

    let expected = vec![32.0, 50.0, 68.0, 86.0, 98.6, 212.0];
    let correct = fahrenheit_result.iter().zip(&expected)
        .all(|(a, b)| (a - b).abs() < 0.1);

    if correct {
        println!("\n✓ Perfect parallel map!");
        println!("🎉 Success! You've mastered the map pattern!");
        println!("\nThis pattern scales perfectly - each element is independent!");
    } else {
        println!("\n❌ Results don't match!");
    }
}
