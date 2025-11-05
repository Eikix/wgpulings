// patterns06.wgsl
//
// Atomic Operations - Thread-Safe Operations
//
// Atomics allow multiple threads to safely modify the same memory.
// Essential for:
// - Histograms
// - Counters
// - Locks
// - Synchronization
//
// Available atomics:
// - atomicAdd, atomicSub
// - atomicMax, atomicMin
// - atomicAnd, atomicOr, atomicXor
// - atomicExchange
// - atomicCompareExchangeWeak
//
// Important: Atomics only work with atomic<i32> or atomic<u32>!
//
// Your task: Use atomics to find the maximum value!
//
// Execute `wgpulings hint patterns06` if you need help!

// I AM NOT DONE

@group(0) @binding(0) var<storage, read> values: array<u32>;
@group(0) @binding(1) var<storage, read_write> max_value: atomic<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < arrayLength(&values)) {
        let value = values[i];

        // TODO: Atomically update max_value if value is larger
        // Use: atomicMax(&max_value, value);
        ____(&____, ____);  // FIX ME!
    }
}

// Why atomics?
// Without atomics, this could happen:
// Thread A reads max=5, sees value=10, wants to write 10
// Thread B reads max=5, sees value=15, wants to write 15
// Thread A writes 10
// Thread B writes 15
// Result: Correct (15)
//
// But also this:
// Thread A reads max=5, sees value=10
// Thread B reads max=5, sees value=15
// Thread B writes 15
// Thread A writes 10  ← OVERWRITES 15!
// Result: Wrong (10)
//
// Atomics prevent this race condition!
