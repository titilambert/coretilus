#!/bin/bash

set -e
RUSTFLAGS="-C target-feature=+crt-static"

# Linux x86_64
rustup target add x86_64-unknown-linux-gnu

# macOS Intel
rustup target add x86_64-apple-darwin

# macOS Apple Silicon
rustup target add aarch64-apple-darwin

# Windows 64 bits
rustup target add x86_64-pc-windows-gnu



# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Windows 64 bits
cargo build --release --target x86_64-pc-windows-gnu

# macOS Intel
cargo build --release --target x86_64-apple-darwin

# macOS ARM
cargo build --release --target aarch64-apple-darwin
