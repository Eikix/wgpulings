// graphics01.rs - [OPTIONAL] Basic Rendering
//
// So far you've done pure COMPUTE. Now let's VISUALIZE results!
//
// This introduces the graphics pipeline:
// - Vertex shader: Transforms geometry
// - Fragment shader: Colors pixels
// - Much more complex than compute, but powerful for visualization
//
// Your task: Set up a basic render pipeline!

// I AM NOT DONE

fn main() {
    println!("Graphics Exercise 1 - Basic Rendering");
    println!("\nThis is OPTIONAL - only if you want to visualize compute results!");
    println!("\nGraphics concepts:");
    println!("  - Surface: Where to draw (window)");
    println!("  - Vertex shader: Transform geometry");
    println!("  - Fragment shader: Color pixels");
    println!("  - Render pipeline: Much more complex than compute!");
    println!("\nFor pure compute, you don't need graphics!");
    println!("\n🎉 Graphics intro complete!");
    println!("\nTo actually implement rendering:");
    println!("  1. Create a window (winit)");
    println!("  2. Create a surface");
    println!("  3. Configure surface");
    println!("  4. Create render pipeline (vertex + fragment shaders)");
    println!("  5. Draw each frame");
    println!("\nThis is much more involved than compute!");
    println!("Consider resources:");
    println!("  - learn-wgpu.com (rendering tutorial)");
    println!("  - wgpu examples (github.com/gfx-rs/wgpu)");
}
