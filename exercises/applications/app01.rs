// app01.rs
//
// Image Processing - Grayscale Conversion
//
// Images are perfect for GPU parallel processing!
// Each pixel can be processed independently.
//
// Grayscale formula:
// gray = 0.299*R + 0.587*G + 0.114*B
// (Weights match human eye sensitivity)
//
// Your task: Convert a color image to grayscale!
//
// Execute `wgpulings hint app01` if you need help!

// I AM NOT DONE

use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let width = 4;
    let height = 4;
    let size = (width * height) as usize;

    // Create a simple test image (red gradient)
    let pixels: Vec<Pixel> = (0..size)
        .map(|i| Pixel {
            r: (i * 16) as u8,
            g: 100,
            b: 50,
            a: 255,
        })
        .collect();

    println!("Converting {}x{} image to grayscale", width, height);

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Image"),
        contents: bytemuck::cast_slice(&pixels),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Image"),
        size: (size * std::mem::size_of::<Pixel>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging"),
        size: (size * std::mem::size_of::<Pixel>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Grayscale"),
        source: wgpu::ShaderSource::Wgsl(r#"
            struct Pixel {
                r: u32,
                g: u32,
                b: u32,
                a: u32,
            }

            @group(0) @binding(0) var<storage, read> input: array<Pixel>;
            @group(0) @binding(1) var<storage, read_write> output: array<Pixel>;

            @compute @workgroup_size(8, 8)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let index = gid.y * 4u + gid.x;  // 4 = width
                if (index < arrayLength(&input)) {
                    let pixel = input[index];

                    // TODO: Calculate grayscale value
                    // Formula: gray = 0.299*r + 0.587*g + 0.114*b
                    let gray = u32(
                        f32(pixel.r) * ____ +  // FIX ME!
                        f32(pixel.g) * ____ +  // FIX ME!
                        f32(pixel.b) * ____    // FIX ME!
                    );

                    output[index] = Pixel(gray, gray, gray, pixel.a);
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
        // 2D dispatch for image processing
        pass.dispatch_workgroups(1, 1, 1);  // (width/8, height/8, 1)
    }

    encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, (size * 4) as u64);
    queue.submit(Some(encoder.finish()));

    let slice = staging_buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);

    let data = slice.get_mapped_range();
    let result: Vec<Pixel> = bytemuck::cast_slice(&data).to_vec();

    println!("\n✓ Conversion complete!");
    println!("First pixel: RGB({}, {}, {}) → Gray({}, {}, {})",
             pixels[0].r, pixels[0].g, pixels[0].b,
             result[0].r, result[0].g, result[0].b);

    println!("\n🎉 Success! You've done GPU image processing!");
}
