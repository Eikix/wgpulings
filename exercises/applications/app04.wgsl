// app04.wgsl
// Compute shader for app04

// I AM NOT DONE

struct Particle {
                pos: vec2<f32>,
                vel: vec2<f32>,
            }

            @group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i < arrayLength(&particles)) {
                    var p = particles[i];

                    // Update position
                    p.pos = p.pos + p.vel * ____;  // FIX ME! dt = 0.01

                    // Bounce off walls
                    if (abs(p.pos.x) > 1.0) { p.vel.x = -p.vel.x; }
                    if (abs(p.pos.y) > 1.0) { p.vel.y = -p.vel.y; }

                    particles[i] = p;
                }
            }
