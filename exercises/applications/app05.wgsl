// app05.wgsl
// Compute shader for app05

// I AM NOT DONE

struct Body {
                pos: vec2<f32>,
                vel: vec2<f32>,
                mass: f32,
            }

            @group(0) @binding(0) var<storage, read> bodies_in: array<Body>;
            @group(0) @binding(1) var<storage, read_write> bodies_out: array<Body>;

            const G: f32 = 0.1;  // Gravitational constant

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
                let i = gid.x;
                if (i >= arrayLength(&bodies_in)) { return; }

                var body = bodies_in[i];
                var force = vec2<f32>(0.0, 0.0);

                // Sum forces from all other bodies
                for (var j = 0u; j < arrayLength(&bodies_in); j++) {
                    if (i == j) { continue; }

                    let other = bodies_in[j];
                    let delta = other.pos - body.pos;
                    let dist_sq = dot(delta, delta) + ____;  // FIX ME! Add 0.01 to avoid singularity
                    let dist = sqrt(dist_sq);

                    // F = G * m1 * m2 / r²
                    force += normalize(delta) * (G * body.mass * other.mass / dist_sq);
                }

                // Update velocity and position
                body.vel += force / body.mass * 0.01;  // dt = 0.01
                body.pos += body.vel * 0.01;

                bodies_out[i] = body;
            }
