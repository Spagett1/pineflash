#!/bin/bash
set -e

echo "Building PineFlash for macOS..."

# Move to project root
cd ..

# Build the release binary
echo "Building release binary..."
cargo build --release

# Move back to macos directory
cd macos

# Run the DMG creation script
echo "Creating app bundle and DMG..."
./create-dmg.sh

echo "Build complete!"