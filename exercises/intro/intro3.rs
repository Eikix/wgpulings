// intro3.rs
//
// Understanding Render Passes
//
// A render pass is how you tell the GPU "I want to draw something now".
// Think of it like:
// 1. Opening a canvas (begin the pass)
// 2. Drawing on it (issue draw commands)
// 3. Finishing (end the pass)
//
// Key components:
// - ColorAttachment: Which texture to draw to (like a canvas)
// - load_op: What to do at the start (Clear = fill with color, Load = keep existing)
// - store_op: What to do at the end (Store = save the result, Discard = throw away)
//
// Your task: Set up a render pass that clears a texture to blue.
//
// Execute `wgpulings hint intro3` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup (you know this now!)
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

    // Create a texture to render to
    // Think of this as creating a blank canvas
    let texture_size = wgpu::Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Texture"),
        size: texture_size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    println!("✓ Created render texture");

    // Create a command encoder
    // This is like a "recording" that you'll submit to the GPU
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // TODO: Create a render pass that clears the texture to blue
    // Fill in the missing parts below

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: ____, // FIX ME! (0.0 to 1.0)
                        g: ____, // FIX ME!
                        b: ____, // FIX ME! (hint: blue should be 1.0)
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::____,  // FIX ME! (Store or Discard?)
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // The render pass ends when _render_pass goes out of scope
    }

    // Submit the commands to the GPU
    queue.submit(Some(encoder.finish()));

    println!("✓ Render pass executed");
    println!("\n🎉 Excellent! You've created and executed your first render pass!");
    println!("You cleared a texture to blue. Next, you'll learn to draw actual shapes!");
}
