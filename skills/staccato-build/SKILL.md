---
name: staccato-build
description: "Build, bootstrap, or version Staccato in this repo. Use when running sb builds (rust/glue/managed/all), setting up the build tool via ./bootstrap.sh, or updating version numbers in Cargo.toml and Directory.Build.props."
---

# Staccato Build

## Overview

Use this skill to guide build, bootstrap, and version-sync work for the Staccato repository.

## Workflow

1. Bootstrap the build tool if needed: run `./bootstrap.sh` from repo root.
2. Build a target with `sb` (`build-rust`, `build-glue`, `build-managed`, `build-all`).
3. Update versions with `sb update-version` instead of editing version files manually.
4. Use the paths in `references/building.md` when you need to locate build inputs.

## References

Read `references/building.md` for command details, versioning notes, and key paths.
