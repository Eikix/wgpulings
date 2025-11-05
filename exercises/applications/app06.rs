// app06.rs - Bitonic Sort
// Parallel sorting algorithm!
// O(log²n) comparisons, fully parallel

// I AM NOT DONE

use wgpu::util::DeviceExt;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let data = vec![5u32, 2, 8, 1, 9, 3, 7, 4];
    println!("Sorting: {:?}", data);

    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        contents: bytemuck::cast_slice(&data),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        label: None,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        source: wgpu::ShaderSource::Wgsl(r#"
            @group(0) @binding(0) var<storage, read_write> data: array<u32>;

            struct Params {
                stage: u32,
                step: u32,
            }

            @group(0) @binding(1) var<uniform> params: Params;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i >= arrayLength(&data) / 2u) { return; }

                let stage = params.stage;
                let step = params.step;

                // Bitonic sort compare-swap
                let pair_dist = 1u << (stage - step);
                let block_size = 2u * pair_dist;

                let left_id = (i / pair_dist) * block_size + (i % pair_dist);
                let right_id = left_id + pair_dist;

                let ascending = ((left_id >> stage) & 1u) == 0u;

                if (right_id < arrayLength(&data)) {
                    let left_val = data[left_id];
                    let right_val = data[right_id];

                    let swap = (left_val > right_val) == ascending;

                    if (swap) {
                        data[left_id] = ____;  // FIX ME! right_val
                        data[right_id] = ____;  // FIX ME! left_val
                    }
                }
            }
        "#.into()),
        label: None,
    });

    println!("🎉 Bitonic sort shader ready!");
    println!("Note: Full implementation needs multiple passes!");
}
