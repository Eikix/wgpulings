// intro01.rs
//
// Welcome to wgpulings - COMPUTE EDITION!
//
// This tutorial teaches GPU parallel computing using wgpu and WGSL.
// We focus on COMPUTE (data processing), NOT graphics (rendering triangles).
//
// GPU Compute is perfect for:
// - Parallel data processing
// - Scientific simulations
// - Image/video processing
// - Machine learning
// - Physics simulations
// - Any problem that can run thousands of operations simultaneously
//
// Your task: Set up wgpu for compute workloads (no window, no rendering!)
//
// Execute `wgpulings hint intro01` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // STEP 1: Create an Instance
    // For compute-only work, we don't need any special backends

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    println!("✓ Created wgpu instance");

    // STEP 2: Request an Adapter
    // An adapter represents your GPU (or software fallback)
    //
    // TODO: Fill in the power preference
    // For compute workloads, HighPerformance is usually what you want!

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::____, // FIX ME!
            compatible_surface: None,                      // No surface needed for compute!
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    println!("✓ Found GPU adapter: {}", adapter.get_info().name);
    println!("  Backend: {:?}", adapter.get_info().backend);

    // STEP 3: Request a Device and Queue
    // Device: Your connection to the GPU
    // Queue: Where you submit compute commands
    //
    // TODO: Uncomment the code below

    // let (device, queue) = adapter
    //     .request_device(
    //         &wgpu::DeviceDescriptor {
    //             label: Some("Compute Device"),
    //             required_features: wgpu::Features::empty(),
    //             required_limits: wgpu::Limits::default(),
    //         },
    //         None,
    //     )
    //     .await
    //     .expect("Failed to create device");

    // println!("✓ Created device and queue");
    // println!("\n🎉 Success! You're ready for GPU compute!");
    // println!("No windows, no triangles - just pure parallel processing power!");
}
