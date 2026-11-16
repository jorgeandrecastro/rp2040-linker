[![Crates.io](https://img.shields.io/crates/v/rp2040-linker.svg)](https://crates.io/crates/rp2040-linker)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
# rp2040-linker

**Automatic linker and boot configuration for the RP2040 (Raspberry Pi Pico) in Rust.**

This crate makes it easy to manage the memory layout and second-stage bootloader (BOOT2) for `no_std` Rust applications. No need to configure your **memory.x** file manually.

✅ Successfully tested with Raspberry Pi Pico and RP2040 Zero.

----

## 🚀 Features

* **Zero configuration:** Automatically injects an optimized `memory.x` for the RP2040 (2 MB Flash, 264 KB RAM).
* **Boot ready:** Handles the `.boot2` section at the correct address (`0x10000000`).
* **Guaranteed placement:** Uses `INSERT BEFORE .text` to ensure the bootloader is placed exactly where the BootROM expects it.
* **Broad compatibility:** Works with `cortex-m-rt`, `embassy-rp`, and `rp2040-hal`.

----

## 📦 Installation

**Add this to your `Cargo.toml` file:**

```toml
[dependencies]
rp2040-linker = "0.2"
```

## 🛠 Usage

In your `main.rs` file, simply import the crate to enable automation:

```rust
#![no_std]
#![no_main]

// The essential line: it handles memory and boot sections.
use rp2040_linker as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Your RP2040 code goes here...
    loop {}
}
```

**Required configuration in `.cargo/config.toml`**

Make sure your project uses the linker arguments from `cortex-m-rt`:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=--nmagic",
]

[build]
target = "thumbv6m-none-eabi"
```

----

# 🏗 Memory Map

The crate defines the following layout:

```
Section   Origin       Size          Description

BOOT2     0x10000000   256 bytes     Second-stage bootloader
FLASH     0x10000100   2047 KB       Application code and data
RAM       0x20000000   264 KB        System SRAM
```

----

# ⚡Workflow: Quick Flash (`flash.sh`)

For those using `picotool` directly, here is an automation script to build, convert, and flash your RP2040 with a single command.

**📄 `flash.sh`**

Create this file in the root of your project:

```bash
#!/bin/bash
# Add Cargo to PATH for sudo environments.
export PATH="$HOME/.cargo/bin:$PATH"

echo "🦀 Step 1: Building in release mode..."
cargo build --release || { echo "❌ Build failed"; exit 1; }

# NOTE: Replace 'your_project' with the name of your binary in Cargo.toml.
BINARY_NAME="your_project"
TARGET_PATH="target/thumbv6m-none-eabi/release/$BINARY_NAME"

echo "📦 Step 2: Converting ELF to UF2 for RP2040..."
picotool uf2 convert -t elf "$TARGET_PATH" firmware.uf2 --family rp2040

echo "⚡ Step 3: Flashing the device (requires sudo)..."
sudo picotool load firmware.uf2 -x

echo "✅ Done! Your Pico is restarting..."
```

**How to use it:**

1. Make the script executable: `chmod +x flash.sh`
2. Flash the device: `./flash.sh`

----

# 🏗️ Why use this crate?

Starting an RP2040 is a strict process. The processor looks for a specific 256-byte boot block called BOOT2 at address `0x10000000`. Without a correct memory layout, your Rust program will not run.

`rp2040-linker` greatly simplifies this process:

* **Invisible infrastructure:** It generates and injects the `memory.x` linker script required during the build.
* **Guaranteed BOOT2 placement:** The `INSERT BEFORE .text` instruction places the boot block before your application code, ensuring compatibility with the BootROM.
* **Target check:** It prevents accidental build errors if you are not targeting the ARM architecture (`thumbv6m-none-eabi`).

----

# 🔄 Conflict handling (priority and shadowing)

**What if I already have a `memory.x` file in the root of my project?**

Rust's linker (`rust-lld`) uses the first memory script it finds. If a local file is present, it may shadow the one provided by this crate, which can prevent the RP2040 from starting.

**🛡️ Detection and isolation**

* **Automatic detection:** The crate checks for a local `memory.x` file in your project.
* **Smart isolation:** The mechanism ignores `rp2040-linker` itself when it is being built (using `CARGO_PKG_NAME`). This prevents false positives for contributors to the library.
* **Targeted warning:** The warning is only emitted when an application project (your firmware) has its own file.

⚠️ **Warning**

If a conflict is detected, a message is displayed during the build:

> ⚠️ A `memory.x` file was detected in the root of your project. It may conflict with `rp2040-linker`.

**✅ Resolution**

To let the crate manage your memory completely (the “zero-config” mode), we strongly recommend removing the local `memory.x` file. This ensures that the crate's optimized configuration takes priority.

**Why is this important?**

It helps keep your build pipeline (CI/CD) clean while protecting developers from unpredictable linker behavior.

----

# 🛡️ Security and reliability

By centralizing memory management in this crate, you avoid human copy-and-paste errors across your projects. You benefit from a solid foundation that has been tested and conforms to Raspberry Pi's technical specifications.

----

# 🔄 Conflict handling (technical note)

Note: Automatic detection of a local `memory.x` file works primarily when developing the crate or when using it through a local path (`path = "..."`).

Due to Cargo's security isolation, when installed from crates.io the crate may not be able to “see” files in your parent project. As a precaution, always remove any manually created `memory.x` file from the root of your project to ensure the crate's file is used and to avoid unexpected errors caused by an incorrect `memory.x` file.

----

## 🛡️ License

Licensed under either of:

    MIT license (http://opensource.org/licenses/MIT)

    Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

at your option.

----

# 🦅 About

Developed by Jorge Andre Castro
