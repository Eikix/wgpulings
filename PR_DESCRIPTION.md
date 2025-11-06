# Pull Request: Add wgpulings - Compute-first GPU Programming Tutorial

## Summary

This PR adds **wgpulings** - an interactive, compute-first tutorial for learning GPU parallel computing with wgpu and WGSL. Inspired by rustlings, it provides 29 hands-on exercises focused on parallel computing and data processing, NOT graphics rendering.

Perfect for learning:

- 🚀 Parallel data processing
- 🔬 Scientific computing
- 🖼️ Image/video processing
- 🤖 Machine learning operations
- ⚛️ Physics simulations
- 📊 Data analytics

## What's Included

### 🎓 Complete Tutorial System (29 Exercises)

**1. INTRO (4 exercises)** - GPU Compute Fundamentals

- Setup wgpu for compute (no windows needed!)
- First compute shader with parallel execution
- Buffers and CPU↔GPU data transfer
- Workgroups and dispatch calculations

**2. BASICS (5 exercises)** - Core Parallel Programming

- WGSL compute shader syntax
- Storage buffers (read/write)
- Uniform buffers (parameters)
- Reading results back to CPU
- Multiple compute passes

**3. PATTERNS (6 exercises)** - Parallel Algorithms

- Parallel Map (element-wise operations)
- Parallel Reduction (sum, max, min)
- Workgroup shared memory
- Prefix sum/scan
- Histogram with atomics
- Atomic operations

**4. PERFORMANCE (5 exercises)** - Optimization

- Workgroup size optimization
- Memory coalescing patterns
- Bank conflict avoidance
- Benchmarking GPU code
- Occupancy optimization

**5. APPLICATIONS (6 exercises)** - Real-World Use Cases

- Image processing (grayscale, blur)
- Matrix multiplication
- Particle simulation
- N-body physics
- GPU sorting (bitonic sort)

**6. GRAPHICS (3 exercises)** - OPTIONAL Visualization

- Basic rendering concepts
- Visualizing compute results
- Animation loops (compute + graphics)

### 🛠️ Interactive CLI Tool

```bash
# View all exercises
cargo run --release -- list

# Watch mode (auto-checks on save)
cargo run --release -- watch

# Run specific exercise
cargo run --release -- run intro01

# Get hints
cargo run --release -- hint intro01

# Verify all
cargo run --release -- verify
```

### 📚 Exercise Features

Each exercise includes:

- ✅ `I AM NOT DONE` marker for progress tracking
- ✅ `____` placeholders for hands-on learning
- ✅ Extensive explanatory comments
- ✅ Comprehensive hints
- ✅ Progressive difficulty
- ✅ Real-world examples

## Why Compute-First?

Traditional GPU tutorials focus on graphics (triangles, rendering, lighting). This tutorial is different:

❌ **NOT focused on:**

- Drawing triangles
- Vertex/fragment shaders
- 3D graphics pipelines
- Rendering techniques

✅ **FOCUSED on:**

- Parallel data processing
- Compute shaders (WGSL)
- Fast parallel algorithms
- Performance optimization
- Real compute applications

## Key Concepts Taught

- **GPU Parallelism**: Thousands of threads running simultaneously
- **Memory Management**: Storage buffers, uniforms, staging buffers
- **Parallel Patterns**: Map, reduce, scan, histogram
- **Performance**: Coalescing, occupancy, workgroup sizing
- **Real Applications**: Image processing, simulations, sorting

## Technical Implementation

**CLI Tool:**

- Built with clap, colored output, progress tracking
- File watcher for auto-checking
- Exercise validation system
- Comprehensive hint system

**Dependencies:**

- wgpu 0.19 (WebGPU implementation)
- pollster (async runtime)
- bytemuck (safe casting)
- Standard Rust ecosystem

**Exercise System:**

- 29 progressive exercises
- Both Rust and WGSL files
- Compile and run modes
- Automatic validation

## Usage

```bash
# Install
git clone https://github.com/Eikix/wgpulings.git
cd wgpulings
cargo build --release

# Start learning
cargo run --release -- watch
```

## Documentation

- **README.md**: Complete tutorial guide with examples
- **CONTRIBUTING.md**: Guidelines for adding exercises
- **info.toml**: Exercise metadata and comprehensive hints
- **Inline comments**: Every exercise heavily documented

## Target Audience

- Software engineers new to GPU programming
- Experienced Rust developers wanting GPU compute
- Anyone needing parallel data processing
- Scientists/researchers needing GPU acceleration
- NO graphics knowledge required!

## Files Added

```
wgpulings/
├── src/
│   ├── main.rs         # CLI entry point
│   ├── exercise.rs     # Exercise loading & validation
│   ├── run.rs          # Exercise execution
│   ├── verify.rs       # Batch verification
│   └── watch.rs        # File watcher
├── exercises/
│   ├── intro/          # 4 exercises
│   ├── basics/         # 5 exercises
│   ├── patterns/       # 6 exercises
│   ├── perf/           # 5 exercises
│   ├── applications/   # 6 exercises
│   └── graphics/       # 3 exercises (optional)
├── info.toml           # Exercise definitions & hints
├── README.md           # Comprehensive guide
├── CONTRIBUTING.md     # Contribution guidelines
├── LICENSE             # MIT
└── Cargo.toml          # Dependencies
```

## Test Plan

- [x] CLI compiles and runs
- [x] All 29 exercises listed correctly
- [x] Hint system works
- [x] List command shows progress
- [x] Exercise structure validated
- [x] Comprehensive README
- [x] All files properly organized

## Learning Path

```
Beginner (Intro + Basics)
    ↓
Intermediate (Patterns + Performance)
    ↓
Advanced (Applications)
    ↓
Optional (Graphics for visualization)
```

## Examples

**Parallel Map (patterns01):**

```rust
// Each thread processes one element independently
output[i] = f(input[i]);
```

**Reduction (patterns02):**

```rust
// Combine many values to one (sum, max, min)
sum = reduce(array, add);
```

**Image Processing (app01):**

```rust
// Convert color to grayscale in parallel
gray = 0.299*r + 0.587*g + 0.114*b;
```

## Benefits

✅ Learn GPU programming from scratch
✅ No graphics knowledge needed
✅ Hands-on, interactive learning
✅ Comprehensive hints for every exercise
✅ Progressive difficulty
✅ Real-world applications
✅ Performance optimization techniques
✅ Modern WebGPU API (wgpu)
✅ Safe Rust practices

## Future Enhancements (Not in this PR)

Potential additions:

- Advanced reduction techniques (tree reduction)
- More complex image filters
- FFT and signal processing
- Ray-tracing compute examples
- ML operations (convolution, etc.)
- More optimization exercises

## Related Resources

- [wgpu Documentation](https://docs.rs/wgpu/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)

---

## Ready to Learn GPU Computing! 🚀

This tutorial provides a complete, structured path from zero to competent in GPU parallel computing. Perfect for anyone wanting to harness GPU power for data processing, simulations, or any parallel workload!

**Start learning:**

```bash
cargo run --release -- watch
```

⚡ Let's harness the power of parallel computing! ⚡
