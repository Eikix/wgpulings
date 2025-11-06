# wgpulings ⚡

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-2021+-orange.svg)

**Learn GPU parallel computing and WGSL compute shaders through hands-on exercises!**

Inspired by [rustlings](https://github.com/rust-lang/rustlings), wgpulings teaches you **GPU compute programming** with Rust's wgpu library and the WebGPU Shading Language (WGSL).

## What is This?

wgpulings is an interactive, **compute-first** tutorial for learning GPU programming. We focus on **parallel computing** and **data processing**, NOT graphics rendering.

Perfect for:

- 🚀 **Parallel data processing** (process millions of elements in parallel)
- 🔬 **Scientific computing** (simulations, numerical methods)
- 🖼️ **Image/video processing** (filters, transformations)
- 🤖 **Machine learning** (matrix operations, neural networks)
- ⚛️ **Physics simulations** (particles, N-body problems)
- 📊 **Data analytics** (aggregations, transformations)

Learn to write code that runs **thousands of operations simultaneously** on the GPU!

## Why GPU Compute?

Modern GPUs have **thousands of cores**. A single GPU can execute:

- **10,000+ threads in parallel**
- **Teraflops of computation**
- **Hundreds of GB/s memory bandwidth**

Perfect for problems where you need to do the same operation on lots of data!

## Why wgpu?

- **Modern**: Based on the WebGPU standard
- **Safe**: Rust's safety guarantees for GPU code
- **Cross-platform**: Works on Windows, macOS, Linux, and web
- **No graphics knowledge needed**: Pure compute, no rendering!
- **Future-proof**: WebGPU is the modern GPU API

## Prerequisites

- **Rust knowledge**: You should be comfortable with Rust. If not, try [rustlings](https://github.com/rust-lang/rustlings) first!
- **No GPU experience required**: We assume you've never written GPU code before.
- **Basic programming**: Arrays, loops, functions, parallel thinking helpful

## Installation

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

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

### Watch mode (recommended!)

```bash
cargo run --release -- watch
```

This is the **best way to learn**! Watch mode:

- ⚡ Automatically checks your solution when you save
- 🔍 Shows compilation errors immediately
- ➡️ Moves to the next exercise when you complete one
- 💡 Provides instant feedback

### Other commands

```bash
# List all exercises
cargo run --release -- list

# Run a specific exercise
cargo run --release -- run intro01

# Get a hint
cargo run --release -- hint intro01

# Verify all exercises
cargo run --release -- verify
```

## Learning Path

### 🎓 INTRO (4 exercises)

Learn GPU compute fundamentals:

- Setting up wgpu for compute (no windows!)
- Your first compute shader
- Buffers and data transfer (CPU ↔ GPU)
- Workgroups and parallel dispatch

### ⚙️ COMPUTE BASICS (5 exercises)

Master core parallel programming:

- WGSL compute shader syntax
- Storage buffers (read/write data)
- Uniform buffers (parameters)
- Reading results back to CPU
- Multiple compute passes

### 🔄 PARALLEL PATTERNS (6 exercises)

Learn common parallel algorithms:

- **Map**: Apply function to every element
- **Reduce**: Combine many values to one (sum, max, min)
- **Scan**: Prefix sum (cumulative operations)
- **Histogram**: Count occurrences
- **Atomics**: Thread-safe operations
- **Shared memory**: Fast workgroup-local memory

### ⚡ PERFORMANCE (5 exercises)

Optimize your GPU code:

- Workgroup sizing strategies
- Memory coalescing patterns
- Bank conflict avoidance
- Benchmarking and profiling
- Occupancy optimization

### 🚀 APPLICATIONS (6 exercises)

Build real-world programs:

- Image processing (grayscale, blur, filters)
- Matrix multiplication (tiled algorithm)
- Particle simulations
- N-body physics
- GPU sorting algorithms
- Custom compute pipelines

### 🎨 GRAPHICS (3 exercises - OPTIONAL)

If you want to visualize results:

- Basic rendering pipeline
- Visualizing compute results
- Mixing compute + graphics

## Exercise Structure

Each exercise:

1. Has the marker `I AM NOT DONE` (remove when solved)
2. Contains `____` markers showing what to fix
3. Includes extensive comments explaining concepts
4. Has a helpful hint (run `hint <name>`)

### Example Exercise Flow

```rust
// intro01.rs - Set up wgpu for compute

// I AM NOT DONE  ← Remove this when done

fn main() {
    // ... setup code ...

    // TODO: Fill in the power preference
    power_preference: wgpu::PowerPreference::____,  // FIX ME!
}
```

1. Read the exercise and comments
2. Fix the `____` markers
3. Remove `I AM NOT DONE`
4. Save → watch mode checks it automatically!

## GPU Compute Concepts

### The GPU Execution Model

```
CPU                           GPU
────                          ────
Upload data    ──────────>    Storage Buffer
               │               (GPU memory)
               │                    │
Submit work    ─────>          Compute Shader
               │               (thousands of threads)
               │                    │
Download       <──────────     Results Buffer
results
```

### Key Terminology

- **Compute Shader**: Program that runs on GPU (in WGSL)
- **Thread**: Single execution of your shader
- **Workgroup**: Group of threads that cooperate
- **Dispatch**: Launch N workgroups
- **Storage Buffer**: Read/write GPU memory
- **Uniform Buffer**: Read-only parameters
- **Barrier**: Synchronization point

### Parallelism Hierarchy

```
Dispatch
├── Workgroup 0 (256 threads)
│   ├── Thread 0
│   ├── Thread 1
│   ├── ...
│   └── Thread 255
├── Workgroup 1 (256 threads)
│   └── ...
└── Workgroup N
    └── ...
```

Each thread has a unique `global_invocation_id` - use it to index your data!

## Common Patterns

### Pattern 1: Parallel Map

```wgsl
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < arrayLength(&input)) {
        output[i] = f(input[i]);  // Process independently
    }
}
```

### Pattern 2: Reduction

```wgsl
// Each workgroup reduces its chunk
var<workgroup> shared: array<f32, 256>;

@compute @workgroup_size(256)
fn reduce(@builtin(global_invocation_id) gid: vec3<u32>,
          @builtin(local_invocation_id) lid: vec3<u32>) {
    // Load data
    shared[lid.x] = input[gid.x];
    workgroupBarrier();

    // Tree reduction
    for (var stride = 128u; stride > 0u; stride >>= 1u) {
        if (lid.x < stride) {
            shared[lid.x] += shared[lid.x + stride];
        }
        workgroupBarrier();
    }

    // Thread 0 writes result
    if (lid.x == 0u) {
        output[workgroup_id] = shared[0];
    }
}
```

## Tips for Success

1. **Go in order**: Exercises build on each other
2. **Read comments**: They explain the concepts
3. **Use hints**: `cargo run -- hint <name>`
4. **Experiment**: Try changing values, see what happens!
5. **Think parallel**: How can each thread work independently?
6. **Watch memory**: Adjacent threads should access adjacent memory
7. **Check bounds**: Always verify `index < array_length`

## Common Pitfalls

1. **Forgetting `I AM NOT DONE`**: Remove it when done!
2. **Wrong workgroup count**: Use `ceil(elements / workgroup_size)`
3. **Out of bounds**: Always check array bounds in shaders
4. **Race conditions**: Use atomics or separate per-workgroup data
5. **Wrong buffer usage**: Storage vs Uniform vs Staging
6. **Forgetting barriers**: Sync before reading shared memory

## Troubleshooting

### "Failed to find adapter"

- Update graphics drivers
- Try software fallback
- Check GPU compatibility

### "Shader compilation failed"

- Read the error message carefully
- Check WGSL syntax
- Verify @group/@binding numbers match

### "Results are wrong"

- Check workgroup dispatch calculation
- Verify array bounds checks
- Print intermediate values (use storage buffer)
- Start with small data sizes

## Resources

### WebGPU & WGSL

- [WebGPU Spec](https://www.w3.org/TR/webgpu/)
- [WGSL Spec](https://www.w3.org/TR/WGSL/)
- [wgpu Docs](https://docs.rs/wgpu/)

### GPU Computing

- [CUDA Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/) (concepts transfer!)
- [GPU Gems](https://developer.nvidia.com/gpugems/gpugems3/contributors) (algorithms)
- [Parallel Algorithms](https://en.wikipedia.org/wiki/Parallel_algorithm) (theory)

### Community

- [Graphics Programming Discord](https://discord.gg/graphicsprogramming)
- [wgpu Matrix Chat](https://matrix.to/#/#wgpu:matrix.org)
- [Rust GPU](https://github.com/EmbarkStudios/rust-gpu)

## Example: Your First Compute Shader

```rust
// Rust side
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    source: wgpu::ShaderSource::Wgsl(r#"
        @group(0) @binding(0) var<storage, read_write> data: array<f32>;

        @compute @workgroup_size(64)
        fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
            let i = gid.x;
            if (i < arrayLength(&data)) {
                data[i] = data[i] * 2.0;  // Double each value!
            }
        }
    "#.into()),
    ..Default::default()
});

// Create pipeline, bind buffers, dispatch...
compute_pass.dispatch_workgroups(num_elements / 64 + 1, 1, 1);
```

This runs 64 threads per workgroup, each doubling one element!

## Contributing

Found a bug? Want to add an exercise? Contributions welcome!

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- Inspired by [rustlings](https://github.com/rust-lang/rustlings)
- Built with [wgpu](https://github.com/gfx-rs/wgpu)
- Thanks to the Rust GPU community!

---

## Ready to Start?

```bash
cargo run --release -- watch
```

**Let's harness the power of parallel computing!** 🚀

Remember: The GPU has thousands of cores waiting to do your bidding. Every pixel you see on screen was computed in parallel. Every machine learning model was trained with massive parallelism. Now it's your turn to write code that scales!

Happy computing! ⚡
