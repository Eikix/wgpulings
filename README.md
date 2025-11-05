# wgpulings 🎨

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-2021+-orange.svg)

Small exercises to get you used to reading and writing **WGSL shaders** and **wgpu** code!

Inspired by [rustlings](https://github.com/rust-lang/rustlings), wgpulings is a hands-on tutorial for learning GPU programming with Rust's wgpu library and the WebGPU Shading Language (WGSL).

## What is This?

wgpulings is an interactive tutorial that teaches you GPU programming from scratch through practical, hands-on exercises. Whether you're a complete GPU beginner or just new to wgpu/WGSL, this tutorial will guide you through:

- 🎯 GPU fundamentals and the graphics pipeline
- 🔺 Drawing your first triangle (the "Hello World" of graphics!)
- 🎨 Vertex and fragment shaders in WGSL
- 📦 Buffers, textures, and GPU memory management
- 🌐 3D graphics, transformations, and cameras
- 💡 Lighting and shading techniques
- ⚡ Advanced topics like compute shaders

## Why wgpu?

- **Modern**: Based on the WebGPU standard
- **Safe**: Rust's safety guarantees extend to GPU code
- **Cross-platform**: Works on Windows, macOS, Linux, and even web browsers
- **Future-proof**: WebGPU is the future of graphics on the web and beyond

## Prerequisites

- **Rust knowledge**: You should be comfortable with Rust. If not, try [rustlings](https://github.com/rust-lang/rustlings) first!
- **No GPU experience required**: This tutorial assumes you've never written GPU code before.

## Installation

### 1. Install Rust

If you haven't already, install Rust from [rustup.rs](https://rustup.rs/)

### 2. Clone this repository

```bash
git clone https://github.com/yourusername/wgpulings.git
cd wgpulings
```

### 3. Build wgpulings

```bash
cargo build --release
```

### 4. Run wgpulings

```bash
cargo run --release -- list
```

## How to Use

wgpulings provides several commands to help you learn:

### List all exercises

```bash
cargo run --release -- list
```

This shows all exercises grouped by topic, with progress indicators.

### Watch mode (recommended!)

```bash
cargo run --release -- watch
```

This is the best way to work through exercises! Watch mode:
- Automatically checks your solution when you save
- Shows compilation errors immediately
- Moves to the next exercise when you complete one
- Provides instant feedback

### Run a specific exercise

```bash
cargo run --release -- run intro1
```

### Get a hint

Stuck? Every exercise has a hint!

```bash
cargo run --release -- hint intro1
```

### Verify all exercises

```bash
cargo run --release -- verify
```

Check all exercises at once to see your overall progress.

## Exercise Structure

All exercises contain the comment `I AM NOT DONE` which you need to remove once you think you've solved the exercise. This prevents accidental completion and encourages deliberate practice.

### How to complete an exercise:

1. Open the exercise file (e.g., `exercises/intro/intro1.rs`)
2. Read the comments and understand what you need to do
3. Fix the `____` markers and complete any TODOs
4. Remove the `I AM NOT DONE` comment
5. Save the file - watch mode will automatically check it!

## Learning Path

The exercises are organized into progressive topics:

### 📚 Intro (3 exercises)
Learn the absolute basics of GPU programming:
- Setting up wgpu (instance, adapter, device, queue)
- Understanding the graphics pipeline
- Your first render pass

### 🎨 Basics (4 exercises)
Start drawing things:
- Clearing the screen
- Drawing your first triangle
- Adding colors to vertices
- Using vertex buffers

### 🖥️ Shaders (5 exercises)
Master WGSL (WebGPU Shading Language):
- WGSL syntax and structure
- Writing vertex shaders
- Writing fragment shaders
- Using uniform buffers
- Passing data between shader stages

### 🖼️ 2D Graphics (4 exercises)
Work with 2D rendering:
- Index buffers for efficient rendering
- Loading and using textures
- Texture sampling and filtering
- Alpha blending and transparency

### 🌍 3D Graphics (6 exercises)
Enter the third dimension:
- 3D coordinate systems
- Transformations (translate, rotate, scale)
- Matrix mathematics
- Camera implementation
- Perspective projection
- Depth testing

### 💡 Lighting (5 exercises)
Make things look realistic:
- Surface normals
- Diffuse lighting (Lambertian)
- Specular highlights
- Combined lighting models (Phong)
- Multiple light sources

### ⚡ Advanced (5 exercises)
Take it to the next level:
- Compute shaders
- Storage buffers
- Instanced rendering
- Cube maps and skyboxes
- Post-processing effects

## Tips for Success

1. **Go in order**: The exercises build on each other. Don't skip ahead!

2. **Read the comments**: Each exercise has extensive comments explaining concepts.

3. **Use hints**: Don't struggle alone - hints are there to help!

4. **Experiment**: Once you complete an exercise, try modifying it:
   - Change colors
   - Adjust positions
   - Add more vertices
   - Break things and see what happens!

5. **Use watch mode**: It's the fastest way to iterate and learn.

6. **Take breaks**: GPU programming involves a lot of new concepts. It's normal to need time to absorb them.

## GPU Programming Concepts

### Key Terms

- **GPU**: Graphics Processing Unit - specialized hardware for parallel computation
- **Shader**: A program that runs on the GPU
- **Vertex**: A point in space (usually a corner of a triangle)
- **Fragment**: A potential pixel (before it's written to the screen)
- **Pipeline**: The series of stages that transform 3D data into pixels
- **Buffer**: A chunk of memory on the GPU
- **Texture**: An image stored in GPU memory
- **Uniform**: Data that's constant for an entire draw call

### The Graphics Pipeline (Simplified)

```
Vertex Data → Vertex Shader → Rasterization → Fragment Shader → Framebuffer
     ↓              ↓              ↓                 ↓              ↓
  Positions    Transform      Convert to        Determine      Final
  Colors       vertices       fragments          colors        image
  UVs, etc.                   (pixels)
```

## Common Pitfalls

1. **Forgetting to remove "I AM NOT DONE"**: The most common mistake! The exercise won't pass until you remove this marker.

2. **Wrong coordinate space**: Remember that clip space goes from -1 to 1, not 0 to 1!

3. **Mismatched @location numbers**: Vertex shader outputs must match fragment shader inputs.

4. **Wrong buffer usage flags**: Make sure you specify the right `BufferUsages` flags.

5. **Forgetting to set the pipeline**: You must call `set_pipeline()` before drawing!

## Troubleshooting

### "Failed to find adapter"

This means wgpu couldn't find a compatible GPU. Try:
- Updating your graphics drivers
- Running in software mode (slower but works everywhere)

### "Compilation failed"

Read the error message carefully:
- For Rust errors: Look at the line number and the error description
- For WGSL errors: Check your shader syntax, especially @location numbers

### "I'm stuck on an exercise"

1. Read the exercise comments again carefully
2. Try the hint: `cargo run --release -- hint <exercise_name>`
3. Look at previous exercises for patterns
4. Take a break and come back with fresh eyes

## Resources

### Learning Resources

- [WebGPU Specification](https://www.w3.org/TR/webgpu/) - The official standard
- [WGSL Specification](https://www.w3.org/TR/WGSL/) - The shader language spec
- [wgpu Documentation](https://docs.rs/wgpu/) - Rust API docs
- [Learn Wgpu](https://sotrh.github.io/learn-wgpu/) - Excellent wgpu tutorial

### Graphics Programming

- [Learn OpenGL](https://learnopengl.com/) - Concepts transfer to any API
- [Scratchapixel](https://www.scratchapixel.com/) - Computer graphics fundamentals
- [Graphics Programming Discord](https://discord.gg/graphicsprogramming) - Community help

## Contributing

Found a bug? Have an idea for a new exercise? Contributions are welcome!

Please open an issue or pull request on GitHub.

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by [rustlings](https://github.com/rust-lang/rustlings)
- Built with [wgpu](https://github.com/gfx-rs/wgpu)
- Thanks to the Rust graphics community!

---

## Ready to Start?

```bash
cargo run --release -- watch
```

Let's learn GPU programming! 🚀

**Remember**: Everyone finds GPU programming confusing at first. Stick with it, and soon you'll be rendering beautiful graphics! The feeling of seeing your first triangle appear on screen is incredible. Enjoy the journey! 🎉
