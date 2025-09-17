#!/bin/bash

# Set the target architecture
TARGET_ARCH=aarch64-unknown-linux-gnu

# Check if rustup is installed
if ! command -v rustup &> /dev/null; then
    echo "rustup is not installed. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# Check if the target architecture is installed in Rust
if ! rustup target list --installed | grep -q "$TARGET_ARCH"; then
    echo "Target $TARGET_ARCH is not installed. Installing..."
    rustup target add $TARGET_ARCH
fi

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "Homebrew is not installed. Please install Homebrew first."
    exit 1
fi

# Ensure the required linker and other tools are available
if ! command -v aarch64-linux-gnu-gcc &> /dev/null; then
    echo "aarch64-linux-gnu-gcc is not installed. Installing..."
    brew tap messense/macos-cross-toolchains
    brew install aarch64-unknown-linux-gnu
fi

# Export the path for the cross-compiler
export CC=aarch64-unknown-linux-gnu-gcc
export CXX=aarch64-unknown-linux-gnu-g++
export AR=aarch64-unknown-linux-gnu-ar
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-unknown-linux-gnu-gcc

# Build the Rust application in release mode for the specified target
cargo build --release --target $TARGET_ARCH

# Check if the build was successful
if [ ! -f target/$TARGET_ARCH/release/minecraft_connector_lambda ]; then
    echo "Build failed. Exiting."
    exit 1
fi

# Create a deployment package
mkdir -p lambda
cp target/$TARGET_ARCH/release/minecraft_connector_lambda lambda/bootstrap

# Zip the deployment package
cd lambda
zip -r9 ../lambda.zip .
cd ..

echo "Build and packaging complete. Deployment package is lambda.zip."
