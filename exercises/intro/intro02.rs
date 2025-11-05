// intro02.rs
//
// Your First Compute Shader - The "Hello World" of GPU Computing!
//
// A compute shader is a program that runs on the GPU in PARALLEL.
// Instead of running once, it runs THOUSANDS of times simultaneously!
//
// Key concept: @workgroup_size
// This defines how many threads run together in a group.
// Think of it like: "Run this function 1000 times in parallel, in groups of 64"
//
// Your task: Create and run your first compute shader!
//
// Execute `wgpulings hint intro02` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup (you learned this in intro01!)
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    println!("✓ GPU ready for compute");

    // TODO: Load and create the shader module
    // The shader is in intro02.wgsl - load it with include_str!()

    // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    //     label: Some("Compute Shader"),
    //     source: wgpu::ShaderSource::Wgsl(include_str!("____").into()),  // FIX ME!
    // });

    // println!("✓ Compute shader compiled");

    // TODO: Create a compute pipeline
    // This is like a "program" that uses your shader

    // let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
    //     label: Some("Compute Pipeline"),
    //     layout: None,
    //     module: &shader,
    //     entry_point: "main",  // Name of the function in the shader
    // });

    // println!("✓ Compute pipeline created");

    // TODO: Run the compute shader!
    // Create a command encoder and dispatch

    // let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
    //     label: Some("Compute Encoder"),
    // });

    // {
    //     let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
    //         label: Some("Compute Pass"),
    //         timestamp_writes: None,
    //     });

    //     compute_pass.set_pipeline(&compute_pipeline);

    //     // Dispatch 10 workgroups
    //     // With workgroup_size(64), this runs 640 threads in parallel!
    //     compute_pass.dispatch_workgroups(10, 1, 1);
    // }

    // queue.submit(Some(encoder.finish()));
    // device.poll(wgpu::Maintain::Wait);

    // println!("\n🎉 Success! You just ran 640 threads in parallel on the GPU!");
    // println!("They didn't do anything useful yet, but they ran!");
}
