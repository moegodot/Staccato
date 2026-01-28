# Architecture Reference

## Module topology (Rust workspace)
- `staccato-core`: zero-dependency math primitives (Rect, Point, Color). Foundation for all crates.
- `staccato-shared`: global error/result conventions and shared constants.
- `staccato-platform-api`: platform-facing contracts (window backend, input, timers).
- `staccato-render-api`: render command protocol; decouples render description from backend.
- `staccato-hal`: SDL3-based platform implementation; OS signal translation.
- `staccato-render-wgpu`: wgpu-based renderer; textures, shaders, pipelines.
- `staccato-engine` (in this repo named `staccato`): aggregates modules and drives Update/FixedUpdate/Render.
- `staccato-dotnet`: FFI boundary between Rust and C#.

Rust crates live under `source/native/rust/<crate_name>`.

## Design principles
- Depend on contracts, not implementations. API crates define interfaces; implementations depend on them.
- Separate state reads (infallible, zero-cost) from state changes (fallible, side-effecting).

## Windowing and rendering lifecycle
- Window object lives in `staccato-hal` and must stay on the main thread (Thread 0).
- Renderer uses a lightweight, copyable, thread-safe window handle; it does not own the window.
- Destroy surfaces before destroying windows. Use explicit teardown (`Option` or `ManuallyDrop`) to avoid self-references.

## Main loop and timing
- Use a fixed timestep accumulator for physics (`FixedUpdate`) and variable timestep for `Update`.
- Guard against the death spiral by capping catch-up work when frame time spikes.
- Prefer SDL3 nanosecond ticks (`SDL_GetTicksNS`) for timestamps on events and input.

## Event system
- Batch events into a contiguous command buffer each frame.
- Expose the buffer to C# via a single FFI call for cache-friendly iteration.

## Telemetry and logging
- Use Rust `tracing` for unified telemetry; bridge C# logs into the same pipeline.
- Allow OpenTelemetry-compatible tooling (e.g., Tracy) for per-frame profiling in dev mode.

## Error handling
- Use precise low-level error enums for recoverable vs fatal failures.
- Convert Rust errors to FFI-friendly error codes + message strings for C#.
- Keep success paths lean; avoid overhead in high-frequency APIs.

## Non-negotiable constraints
- SDL event polling and window ops must stay on Thread 0 (macOS will crash otherwise).
- Avoid self-referential window/surface structs.
- Do not allocate per-frame in Update/Render; pre-allocate and reuse buffers.
- Prefer zero-copy FFI via `repr(C)` shared structs.
- Design every system to work in headless mode (`SDL_VIDEODRIVER=dummy`).
