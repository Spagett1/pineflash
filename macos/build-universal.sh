#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building PineFlash Universal Binary for macOS...${NC}"

# Move to project root
cd ..

# Install both targets if not already installed
echo "Ensuring Rust targets are installed..."
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for Intel
echo -e "${YELLOW}Building for Intel (x86_64)...${NC}"
cargo build --release --target x86_64-apple-darwin

# Build for Apple Silicon
echo -e "${YELLOW}Building for Apple Silicon (ARM64)...${NC}"
cargo build --release --target aarch64-apple-darwin

# Create universal binary
echo -e "${YELLOW}Creating universal binary...${NC}"
lipo -create -output target/release/pineflash-universal \
    target/x86_64-apple-darwin/release/pineflash \
    target/aarch64-apple-darwin/release/pineflash

# Verify the universal binary
echo "Universal binary architectures:"
lipo -info target/release/pineflash-universal

# Copy to standard location
cp target/release/pineflash-universal target/release/pineflash

# Move back to macos directory
cd macos

# Run the DMG creation script
echo -e "${YELLOW}Creating app bundle and DMG...${NC}"
./create-dmg.sh

# Get version from Cargo.toml
VERSION=$(grep '^version' ../Cargo.toml | sed 's/.*"\(.*\)".*/\1/' || echo "0.5.5")

# Rename the output to indicate it's universal
if [ -f "PineFlash-${VERSION}-macOS.dmg" ]; then
    mv "PineFlash-${VERSION}-macOS.dmg" "PineFlash-${VERSION}-macOS-universal.dmg"
    echo -e "${GREEN}Universal DMG created: PineFlash-${VERSION}-macOS-universal.dmg${NC}"
fi

echo -e "${GREEN}Universal build complete!${NC}"