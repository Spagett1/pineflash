#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Creating macOS installer for PineFlash...${NC}"

# Variables
APP_NAME="PineFlash"
BINARY_NAME="pineflash"
VERSION=$(grep '^version' ../Cargo.toml | sed 's/.*"\(.*\)".*/\1/' || echo "0.5.5")
BUILD_DIR="../target/release"
APP_DIR="$APP_NAME.app"
DMG_NAME="PineFlash-$VERSION-macOS.dmg"
ICON_FILE="../assets/pine64logo.png"

# Check if binary exists
if [ ! -f "$BUILD_DIR/$BINARY_NAME" ]; then
    echo -e "${RED}Error: Binary not found at $BUILD_DIR/$BINARY_NAME${NC}"
    echo "Please run 'cargo build --release' first"
    exit 1
fi

# Clean up any existing app bundle
if [ -d "$APP_DIR" ]; then
    echo "Removing existing app bundle..."
    rm -rf "$APP_DIR"
fi

# Create app bundle structure
echo "Creating app bundle structure..."
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy binary
echo "Copying binary..."
cp "$BUILD_DIR/$BINARY_NAME" "$APP_DIR/Contents/MacOS/"
chmod +x "$APP_DIR/Contents/MacOS/$BINARY_NAME"

# Copy Info.plist
echo "Copying Info.plist..."
cp Info.plist "$APP_DIR/Contents/"

# Create icon set if PNG exists
if [ -f "$ICON_FILE" ]; then
    echo "Creating icon set..."
    mkdir -p "$APP_DIR/Contents/Resources/AppIcon.iconset"
    
    # Generate different icon sizes
    sips -z 16 16     "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_16x16.png" > /dev/null 2>&1
    sips -z 32 32     "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_16x16@2x.png" > /dev/null 2>&1
    sips -z 32 32     "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_32x32.png" > /dev/null 2>&1
    sips -z 64 64     "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_32x32@2x.png" > /dev/null 2>&1
    sips -z 128 128   "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_128x128.png" > /dev/null 2>&1
    sips -z 256 256   "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_128x128@2x.png" > /dev/null 2>&1
    sips -z 256 256   "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_256x256.png" > /dev/null 2>&1
    sips -z 512 512   "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_256x256@2x.png" > /dev/null 2>&1
    sips -z 512 512   "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_512x512.png" > /dev/null 2>&1
    sips -z 1024 1024 "$ICON_FILE" --out "$APP_DIR/Contents/Resources/AppIcon.iconset/icon_512x512@2x.png" > /dev/null 2>&1
    
    # Create icns file
    iconutil -c icns "$APP_DIR/Contents/Resources/AppIcon.iconset" -o "$APP_DIR/Contents/Resources/AppIcon.icns"
    rm -rf "$APP_DIR/Contents/Resources/AppIcon.iconset"
else
    echo -e "${YELLOW}Warning: Icon file not found at $ICON_FILE${NC}"
fi

echo -e "${GREEN}App bundle created successfully!${NC}"

# Create DMG
if command -v create-dmg &> /dev/null; then
    echo "Creating DMG installer..."
    
    # Remove old DMG if exists
    [ -f "$DMG_NAME" ] && rm "$DMG_NAME"
    
    # Create DMG with create-dmg tool
    create-dmg \
        --volname "$APP_NAME" \
        --volicon "$APP_DIR/Contents/Resources/AppIcon.icns" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "$APP_NAME.app" 150 150 \
        --hide-extension "$APP_NAME.app" \
        --app-drop-link 450 150 \
        --no-internet-enable \
        "$DMG_NAME" \
        "$APP_DIR"
    
    echo -e "${GREEN}DMG created: $DMG_NAME${NC}"
else
    echo -e "${YELLOW}Warning: create-dmg not found. Install with: brew install create-dmg${NC}"
    echo "Creating simple DMG..."
    
    # Create a simple DMG without fancy styling
    hdiutil create -volname "$APP_NAME" -srcfolder "$APP_DIR" -ov -format UDZO "$DMG_NAME"
    echo -e "${GREEN}Simple DMG created: $DMG_NAME${NC}"
fi

echo -e "${GREEN}Build complete!${NC}"
echo "App bundle: $APP_DIR"
echo "DMG: $DMG_NAME"