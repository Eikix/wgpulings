// intro1.rs
//
// Welcome to wgpulings! This is your first exercise in learning GPU programming
// with wgpu (the Rust implementation of WebGPU) and WGSL (WebGPU Shading Language).
//
// GPU programming is fundamentally different from CPU programming:
// - The GPU excels at parallel processing (doing many things at once)
// - The GPU has its own memory separate from CPU RAM
// - You write "shaders" that run on the GPU
// - Data flows: CPU → GPU memory → Shader execution → Results back to CPU
//
// In this exercise, you'll learn about the basic structure of a GPU application.
// Your task: Read through the code and fix the markers to understand each component.
//
// Key concepts:
// - Instance: The entry point to wgpu
// - Adapter: Represents a physical GPU
// - Device: A logical connection to the GPU
// - Queue: Used to submit commands to the GPU
//
// Execute `wgpulings hint intro1` if you need help!

// I AM NOT DONE

fn main() {
    // The pollster crate allows us to run async code in a synchronous main function
    pollster::block_on(run());
}

async fn run() {
    // STEP 1: Create an Instance
    // The Instance is the entry point to wgpu. It represents the wgpu context.
    // Think of it as "initializing the GPU library".

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(), // Try all available backends (Vulkan, Metal, DX12, etc.)
        ..Default::default()
    });

    println!("✓ Created wgpu instance");

    // STEP 2: Request an Adapter
    // An Adapter represents a physical GPU (or software renderer).
    // You might have multiple GPUs (integrated + discrete), so we request one.
    //
    // TODO: Fill in the PowerPreference below
    // Options: LowPower (integrated GPU), HighPerformance (discrete GPU), or None
    // Hint: For learning, HighPerformance is usually better, but LowPower works too!

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::____, // FIX ME!
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    println!("✓ Found GPU adapter: {}", adapter.get_info().name);

    // STEP 3: Request a Device and Queue
    // Device: A logical connection to the GPU. You use this to create resources.
    // Queue: Used to submit commands (draw calls, compute, etc.) to the GPU.
    //
    // TODO: Uncomment the lines below to request a device and queue

    // let (device, queue) = adapter
    //     .request_device(
    //         &wgpu::DeviceDescriptor {
    //             label: Some("Main Device"),
    //             required_features: wgpu::Features::empty(),
    //             required_limits: wgpu::Limits::default(),
    //         },
    //         None,
    //     )
    //     .await
    //     .expect("Failed to create device");

    // println!("✓ Created device and queue");
    // println!("\n🎉 Congratulations! You've successfully set up wgpu!");
    // println!("You now have a connection to the GPU and can start sending it commands.");
}
