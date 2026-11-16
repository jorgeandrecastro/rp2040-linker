// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.
use std::env;
use std::fs;
use std::path::PathBuf;


/// The build script for `rp2040-linker`.
///
/// This script automatically injects the `memory.x` file into the user's
/// project linking process.
fn main() {
    // Retrieve the output directory for the current build (managed by Cargo).
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not set"));
    let memory_x_path = out.join("memory.x");
    // 1. Write the crate's memory.x file to the build directory.
    // This provides a standard memory configuration with no extra setup.
    fs::write(&memory_x_path, include_bytes!("memory.x"))
        .expect("Failed to write memory.x");

    // 2. Add the output directory to the linker's search path.
    // This lets the linker find our memory.x automatically.
    println!("cargo:rustc-link-search={}", out.display());

    // 3. Tell Cargo to rerun the build script only if build.rs or memory.x changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");

    // 4. Detect conflicts:
    // Only warn for user projects, not when building rp2040-linker itself.
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    let project_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    if pkg_name != "rp2040-linker" && project_dir.join("memory.x").exists() {
        println!("cargo:warning=⚠️ A memory.x file was detected in the root of your project. It may conflict with rp2040-linker.");
    }
}