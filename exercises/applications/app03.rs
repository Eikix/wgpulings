// app03.rs - Matrix Multiplication
// Matrix mult is THE classic parallel algorithm!
// C[i,j] = sum_k (A[i,k] * B[k,j])
//
// Your task: Naive matrix multiplication!

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let n = 4u32;  // 4x4 matrices

    // A = identity, B = [[2, 2, 2, 2], ...]
    let a: Vec<f32> = (0..n*n).map(|i| if i % (n+1) == 0 { 1.0 } else { 0.0 }).collect();
    let b: Vec<f32> = vec![2.0; (n*n) as usize];

    println!("Multiplying {}x{} matrices", n, n);

    let a_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: bytemuck::cast_slice(&a),
        usage: wgpu::BufferUsages::STORAGE,
        label: None,
    });

    let b_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: bytemuck::cast_slice(&b),
        usage: wgpu::BufferUsages::STORAGE,
        label: None,
    });

    let c_buf = device.create_buffer(&wgpu::BufferDescriptor {
        size: (n*n * 4) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
        label: None,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read> a: array<f32>;
            @group(0) @binding(1) var<storage, read> b: array<f32>;
            @group(0) @binding(2) var<storage, read_write> c: array<f32>;

            const N: u32 = 4u;

            @compute @workgroup_size(4, 4)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let row = gid.y;
                let col = gid.x;

                if (row >= N || col >= N) { return; }

                var sum = 0.0;
                for (var k = 0u; k < N; k++) {
                    sum += a[row * N + k] * b[k * N + ____];  // FIX ME!
                }

                c[row * N + col] = sum;
            }
        "#.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
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

    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: a_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: b_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: c_buf.as_entire_binding() },
        ],
        label: None,
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
        label: None,
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&layout),
        module: &shader,
        entry_point: "main",
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bg, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    queue.submit(Some(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);

    println!("✓ Matrix multiplication complete!");
    println!("🎉 You've done GPU matrix multiply!");
}
