---
name: staccato-architecture
description: "Architecture guidance for the Staccato engine. Use when changing module boundaries, windowing/rendering lifecycles, event loop timing, FFI edges, or performance-critical rules in the Rust/C# codebase."
---

# Staccato Architecture

## Overview

Use this skill to keep code changes aligned with the engine's layered design and runtime invariants.

## How to use

1. Read `references/architecture.md` before modifying core engine modules or cross-language boundaries.
2. Preserve module ownership: API crates define contracts; implementation crates depend on them.
3. Maintain runtime rules (main-thread SDL, zero allocations in the main loop, surface-before-window teardown).
4. Keep headless and FFI constraints in mind for any system-level changes.

## References

Read `references/architecture.md` for module topology, engine rules, and design constraints.
