# Build and Version Reference

## Bootstrap
- Run `./bootstrap.sh` from repo root to install the `sb` build tool.

## Build commands
- `sb build-rust`: build the Rust workspace.
- `sb build-glue`: build the Rust/.NET interop layer.
- `sb build-managed`: build the .NET (C#) projects.
- `sb build-all`: build everything.

## Versioning
- Run `sb update-version` to keep version numbers aligned.
- Avoid manual edits to these files:
  - `source/native/rust/Cargo.toml`
  - `source/managed/Directory.Build.props`
  - `staccato.version`

## Key paths
- Rust workspace: `source/native/rust`
- .NET solution: `source/managed/Staccato.Managed.slnx`
- Build tool sources: `building`
