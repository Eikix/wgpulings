// intro2.rs
//
// Now that you understand the basic setup, let's learn about the graphics pipeline.
//
// The GPU graphics pipeline is a sequence of stages that transform 3D data into pixels:
//
// 1. Vertex Shader: Processes each vertex (corner of a triangle)
//    - Input: Vertex attributes (position, color, texture coordinates)
//    - Output: Clip-space position and data to pass to fragment shader
//
// 2. Rasterization: Converts triangles into pixels (fragments)
//    - Happens automatically between vertex and fragment shader
//    - Interpolates vertex data across the triangle
//
// 3. Fragment Shader: Determines the color of each pixel
//    - Input: Interpolated data from vertex shader
//    - Output: Final pixel color
//
// 4. Output: Pixels are written to a texture (usually the screen)
//
// Your task: Complete the shader module creation below.
//
// Execute `wgpulings hint intro2` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Set up instance, adapter, device, and queue (you learned this in intro1!)
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find adapter");

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Main Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    println!("✓ GPU setup complete");

    // SHADERS: Programs that run on the GPU
    // This is a simple shader that draws a red triangle
    let shader_source = r#"
        // Vertex shader: Runs once per vertex
        @vertex
        fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
            // Define a triangle in clip space
            var positions = array<vec2<f32>, 3>(
                vec2<f32>(0.0, 0.5),    // Top
                vec2<f32>(-0.5, -0.5),  // Bottom left
                vec2<f32>(0.5, -0.5)    // Bottom right
            );

            return vec4<f32>(positions[vertex_index], 0.0, 1.0);
        }

        // Fragment shader: Runs once per pixel
        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color (RGBA)
        }
    "#;

    // TODO: Create a shader module from the source code
    // Hint: Use device.create_shader_module() with wgpu::ShaderModuleDescriptor
    // The descriptor needs a label and source (use wgpu::ShaderSource::Wgsl)

    // let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    //     label: Some("Triangle Shader"),
    //     source: wgpu::ShaderSource::Wgsl(____), // FIX ME! (hint: use .into())
    // });

    // println!("✓ Shader compiled successfully");
    // println!("\n🎉 Great! You've created your first shader!");
    // println!("Shaders are programs written in WGSL that run on the GPU.");
    // println!("Next, you'll learn how to use these shaders to actually draw something!");
}
