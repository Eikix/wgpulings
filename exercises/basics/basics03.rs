// basics03.rs
//
// Adding Colors to Vertices
//
// So far, our triangle was a solid color. But what if each corner has a different color?
// The GPU automatically interpolates (blends) colors across the triangle surface!
//
// This introduces "varying" data - data that "varies" from vertex to fragment shader.
//
// Your task: Create a triangle with red, green, and blue corners!
//
// Execute `wgpulings hint basics03` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Texture"),
        size: wgpu::Extent3d { width: 512, height: 512, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Shader with per-vertex colors
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Color Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            // Structure to pass data from vertex to fragment shader
            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) color: vec3<f32>,  // Color will be interpolated!
            }

            @vertex
            fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
                var positions = array<vec2<f32>, 3>(
                    vec2<f32>(0.0, 0.5),
                    vec2<f32>(-0.5, -0.5),
                    vec2<f32>(0.5, -0.5)
                );

                // TODO: Define colors for each vertex
                // Vertex 0 (top) = red (1, 0, 0)
                // Vertex 1 (left) = green (0, 1, 0)
                // Vertex 2 (right) = blue (0, 0, 1)
                var colors = array<vec3<f32>, 3>(
                    vec3<f32>(____, ____, ____),  // Top - Red
                    vec3<f32>(____, ____, ____),  // Left - Green
                    vec3<f32>(____, ____, ____)   // Right - Blue
                );

                var output: VertexOutput;
                output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
                output.color = colors[vertex_index];
                return output;
            }

            @fragment
            fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                // Use the interpolated color!
                return vec4<f32>(input.color, 1.0);
            }
        "#.into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Color Triangle Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
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

        render_pass.set_pipeline(&pipeline);
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));

    println!("✓ Colorful triangle drawn!");
    println!("\n🎉 Beautiful! Notice how the colors blend smoothly across the triangle?");
    println!("That's the GPU doing automatic interpolation for you!");
}
