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
        source: wgpu::ShaderSource::Wgsl(include_str!("app06.wgsl").into()),
        label: None,
    });

    println!("🎉 Bitonic sort shader ready!");
    println!("Note: Full implementation needs multiple passes!");
}
