# Contributing to wgpulings

Thank you for your interest in contributing to wgpulings! This document will help you get started.

## Ways to Contribute

- 🐛 Report bugs or issues with exercises
- 📝 Improve documentation and hints
- ✨ Add new exercises
- 🔧 Fix bugs in the CLI tool
- 💡 Suggest improvements

## Adding New Exercises

### Exercise File Structure

Each exercise should:
1. Have clear learning objectives
2. Include explanatory comments
3. Contain `I AM NOT DONE` marker
4. Use `____` to mark places that need fixing
5. Have `TODO:` comments for tasks
6. Print success messages when complete

### Example Exercise Template (Rust)

```rust
// exercisename.rs
//
// Brief description of what this exercise teaches
//
// Learning objectives:
// - Concept 1
// - Concept 2
//
// Your task: Clear description of what to do
//
// Execute `wgpulings hint exercisename` if you need help!

// I AM NOT DONE

fn main() {
    pollster::block_on(run());
}

async fn run() {
    // Setup code...

    // TODO: Student task description
    let value = ____;  // FIX ME! Explanation

    println!("✓ Success message!");
}
```

### Example Exercise Template (WGSL)

```wgsl
// exercisename.wgsl
//
// Brief description
//
// Learning objectives
//
// Your task: Description

// I AM NOT DONE

@vertex
fn vs_main() -> @builtin(position) vec4<f32> {
    // TODO: Description
    return vec4<f32>(____, ____, ____, ____);  // FIX ME!
}
```

### Adding to info.toml

Add your exercise to `info.toml`:

```toml
[[exercises]]
name = "exercisename"
path = "exercises/category/exercisename.rs"
mode = "compile"  # or "run"
hint = """
Your hint text here.
Multiple lines are fine.
Include:
- What the concept means
- Common mistakes
- Helpful tips
"""
```

### Hint Writing Guidelines

Good hints should:
- Explain the concept, not give the answer
- Point to common mistakes
- Provide encouragement
- Reference related exercises
- Include code examples if helpful

Example:
```
Good: "Uniforms are declared with @group(N) @binding(M). The numbers must match your Rust code."
Bad: "Put @group(0) @binding(0) on line 5"
```

## Testing Your Exercise

1. Add the exercise files
2. Update `info.toml`
3. Run `cargo run -- list` to see it appear
4. Test with `cargo run -- run <name>`
5. Verify the hint: `cargo run -- hint <name>`
6. Try to solve it yourself!

## Code Style

### Rust Code
- Follow standard Rust formatting (`cargo fmt`)
- Use descriptive variable names
- Comment complex sections
- Keep examples simple and focused

### WGSL Code
- Use consistent indentation (4 spaces)
- Comment shader stages clearly
- Explain non-obvious vector math
- Keep variable names descriptive

## Exercise Difficulty

### Beginner
- One concept at a time
- Lots of comments
- Small, focused tasks
- Immediate feedback

### Intermediate
- Combine 2-3 concepts
- Less hand-holding
- Slightly larger tasks
- Encourage experimentation

### Advanced
- Multiple concepts
- Minimal hints
- Open-ended problems
- Optimization challenges

## Topics We Need

Current gaps (feel free to fill these!):
- More texture exercises (mipmaps, texture arrays)
- Animation and time-based effects
- More compute shader examples
- Advanced rendering techniques (deferred, PBR)
- Debugging and profiling exercises
- Performance optimization exercises

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b add-texture-exercise`)
3. Make your changes
4. Test thoroughly
5. Commit with clear messages
6. Push and create a PR

### PR Checklist

- [ ] Exercise file created
- [ ] `info.toml` updated
- [ ] Hint is helpful and clear
- [ ] Tested with `cargo run -- run <name>`
- [ ] Exercise compiles when completed
- [ ] Comments are clear and educational
- [ ] Follows the existing style

## Questions?

Feel free to:
- Open an issue for discussion
- Ask in your PR
- Reach out to maintainers

Thank you for helping make wgpulings better! 🎉
