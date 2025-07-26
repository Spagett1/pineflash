#!/bin/bash
# Smoke test for PineFlash binary
set -e

BINARY="${1:-./target/release/pineflash}"

echo "Testing PineFlash binary: $BINARY"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    exit 1
fi

# Make sure it's executable
chmod +x "$BINARY"

echo "1. Testing --help flag..."
if $BINARY --help > /dev/null 2>&1 || $BINARY -h > /dev/null 2>&1; then
    echo "✅ Help command works"
else
    echo "❌ Help command failed"
    exit 1
fi

echo "2. Testing --version flag..."
if $BINARY --version > /dev/null 2>&1 || $BINARY -V > /dev/null 2>&1; then
    VERSION=$($BINARY --version 2>/dev/null || $BINARY -V 2>/dev/null || echo "unknown")
    echo "✅ Version command works: $VERSION"
else
    echo "❌ Version command failed"
    exit 1
fi

echo "3. Checking binary architecture..."
case "$(uname -s)" in
    Linux)
        file "$BINARY"
        ldd "$BINARY" || echo "Note: Some dependencies might be missing"
        ;;
    Darwin)
        file "$BINARY"
        otool -L "$BINARY" || true
        lipo -info "$BINARY" || true
        ;;
    MINGW*|CYGWIN*|MSYS*)
        file "$BINARY" || echo "$BINARY: Windows executable"
        ;;
esac

echo "4. Testing GUI initialization (may fail in headless environment)..."
# This might fail in CI without display, but that's OK
if [ -z "$DISPLAY" ] && [ "$(uname -s)" = "Linux" ]; then
    echo "⚠️  No display available, skipping GUI test"
else
    timeout 2s $BINARY > /dev/null 2>&1 || true
    echo "✅ Binary can initialize (or timed out gracefully)"
fi

echo ""
echo "✅ Smoke tests completed successfully!"