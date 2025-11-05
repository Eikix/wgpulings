// basics01.rs
//
// Your First Clear: Filling the Screen with Color
//
// Clearing the screen is the simplest GPU operation. Every frame of a game or
// application typically starts by clearing to a background color.
//
// In this exercise, you'll learn:
// - How to create a texture (your "canvas")
// - How to clear it to a specific color
// - Understanding RGBA color values (0.0 to 1.0)
//
// Your task: Clear the screen to a nice cyan color!
//
// Execute `wgpulings hint basics01` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // GPU Setup
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
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    // Create a texture (our canvas)
    let size = 512;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Canvas"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Create command encoder
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Clear Encoder"),
    });

    // TODO: Create a render pass that clears to cyan (0.0, 1.0, 1.0)
    // Cyan = Full green + Full blue, no red
    // Remember: RGBA values go from 0.0 (none) to 1.0 (full)

    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: ____, // Red: 0.0 for cyan
                        g: ____, // Green: 1.0 for cyan
                        b: ____, // Blue: 1.0 for cyan
                        a: 1.0,  // Alpha: always 1.0 (fully opaque)
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
    }

    queue.submit(Some(encoder.finish()));

    println!("✓ Screen cleared to cyan!");
    println!("\n🎉 Success! You've cleared your first frame!");
    println!("Every game does this thousands of times per second!");
}
