// basics02.rs
//
// Drawing Your First Triangle - The "Hello World" of GPU Programming!
//
// Why triangles? Because EVERYTHING in 3D graphics is made of triangles!
// Characters, buildings, terrain - all triangles.
//
// To draw a triangle, you need:
// 1. A vertex shader (processes the 3 corners)
// 2. A fragment shader (colors the pixels inside)
// 3. A render pipeline (connects everything together)
//
// Your task: Complete the pipeline creation and draw a triangle!
//
// Execute `wgpulings hint basics02` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // GPU Setup
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    // Create render target
    let size = 512;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Texture"),
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

    // Shader: A simple triangle that doesn't need vertex buffers
    // It generates positions directly in the vertex shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            @vertex
            fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
                // Three points of a triangle in clip space (-1 to 1)
                var positions = array<vec2<f32>, 3>(
                    vec2<f32>(0.0, 0.5),    // Top center
                    vec2<f32>(-0.5, -0.5),  // Bottom left
                    vec2<f32>(0.5, -0.5)    // Bottom right
                );
                return vec4<f32>(positions[vertex_index], 0.0, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange color
            }
        "#.into()),
    });

    // TODO: Create the render pipeline
    // A pipeline defines HOW to draw: which shaders, what format, etc.

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: None,  // Auto layout
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",  // Name of vertex shader function
            buffers: &[],            // No vertex buffers yet
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "____",     // FIX ME! What's the fragment function called?
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,  // We're drawing triangles
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,  // Counter-clockwise = front
            cull_mode: Some(wgpu::Face::Back), // Don't draw back-facing triangles
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    // Create command encoder and draw!
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Draw Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // TODO: Set the pipeline and draw 3 vertices
        render_pass.set_pipeline(&pipeline);
        render_pass.draw(0..__, 0..1);  // FIX ME! How many vertices in a triangle?
    }

    queue.submit(Some(encoder.finish()));

    println!("✓ Triangle drawn!");
    println!("\n🎉 Congratulations! You've drawn your first triangle!");
    println!("This is a HUGE milestone in GPU programming!");
}
