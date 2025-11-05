// graphics2d01.rs
//
// Index Buffers - Reusing Vertices Efficiently
//
// Drawing a rectangle requires 2 triangles = 6 vertices.
// But a rectangle only has 4 corners!
//
// Index buffers let you reuse vertices:
// - Define 4 vertices for the corners
// - Use indices to specify which vertices form which triangles
// - Saves memory and processing!
//
// Your task: Draw a rectangle using 4 vertices and 6 indices!

// I AM NOT DONE

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

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

    // Define 4 vertices for a rectangle
    let vertices = [
        Vertex { position: [-0.5, 0.5], color: [1.0, 0.0, 0.0] },   // Top-left - Red
        Vertex { position: [0.5, 0.5], color: [0.0, 1.0, 0.0] },    // Top-right - Green
        Vertex { position: [0.5, -0.5], color: [0.0, 0.0, 1.0] },   // Bottom-right - Blue
        Vertex { position: [-0.5, -0.5], color: [1.0, 1.0, 0.0] },  // Bottom-left - Yellow
    ];

    // TODO: Define indices to form 2 triangles
    // Triangle 1: top-left (0), top-right (1), bottom-right (2)
    // Triangle 2: top-left (0), bottom-right (2), bottom-left (3)
    let indices: [u16; 6] = [
        0, 1, 2,  // First triangle
        ____, ____, ____  // FIX ME! Second triangle
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // TODO: Create the index buffer
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::____,  // FIX ME! What usage for index buffer?
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(r#"
            struct VertexInput {
                @location(0) position: vec2<f32>,
                @location(1) color: vec3<f32>,
            }
            struct VertexOutput {
                @builtin(position) position: vec4<f32>,
                @location(0) color: vec3<f32>,
            }
            @vertex
            fn vs_main(input: VertexInput) -> VertexOutput {
                var output: VertexOutput;
                output.position = vec4<f32>(input.position, 0.0, 1.0);
                output.color = input.color;
                return output;
            }
            @fragment
            fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                return vec4<f32>(input.color, 1.0);
            }
        "#.into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                ],
            }],
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
        primitive: wgpu::PrimitiveState::default(),
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
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);  // Draw using indices!
    }

    queue.submit(Some(encoder.finish()));

    println!("✓ Rectangle drawn with index buffer!");
    println!("\n🎉 Great! You used 4 vertices instead of 6 - 33% more efficient!");
}
