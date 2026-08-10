#!/bin/bash
set -e

# Exit immediately if any command fails
echo "=== Starting macOS packaging process for SnITCH ==="

# 1. Clean previous build artifacts
echo "Cleaning old build artifacts..."
rm -rf dist
mkdir -p dist

# 2. Build the release binary
echo "Compiling the application in release mode..."
cargo build --release

# Check if target binary exists
BINARY_PATH="target/release/SnITCH"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Release binary not found at $BINARY_PATH"
    exit 1
fi

# 3. Generate high-resolution .icns file using macOS tools
ICON_SOURCE="images/icon.png"
if [ -f "$ICON_SOURCE" ]; then
    echo "Creating AppIcon.icns from $ICON_SOURCE..."
    ICONSET_DIR="dist/SnITCH.iconset"
    mkdir -p "$ICONSET_DIR"
    
    # Generate various icon resolutions
    sips -z 16 16     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_16x16.png" > /dev/null 2>&1
    sips -z 32 32     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_16x16@2x.png" > /dev/null 2>&1
    sips -z 32 32     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_32x32.png" > /dev/null 2>&1
    sips -z 64 64     "$ICON_SOURCE" --out "$ICONSET_DIR/icon_32x32@2x.png" > /dev/null 2>&1
    sips -z 128 128   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_128x128.png" > /dev/null 2>&1
    sips -z 256 256   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_128x128@2x.png" > /dev/null 2>&1
    sips -z 256 256   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_256x256.png" > /dev/null 2>&1
    sips -z 512 512   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_256x256@2x.png" > /dev/null 2>&1
    sips -z 512 512   "$ICON_SOURCE" --out "$ICONSET_DIR/icon_512x512.png" > /dev/null 2>&1
    sips -z 1024 1024 "$ICON_SOURCE" --out "$ICONSET_DIR/icon_512x512@2x.png" > /dev/null 2>&1
    
    iconutil -c icns "$ICONSET_DIR" -o "dist/AppIcon.icns"
    rm -rf "$ICONSET_DIR"
else
    echo "Warning: Icon source not found at $ICON_SOURCE. Application will use default icon."
fi

# 4. Construct the .app bundle structure
echo "Constructing SnITCH.app bundle..."
APP_DIR="dist/SnITCH.app"
CONTENTS_DIR="$APP_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# Copy the binary
cp "$BINARY_PATH" "$MACOS_DIR/SnITCH"
chmod +x "$MACOS_DIR/SnITCH"

# Copy icon if generated
if [ -f "dist/AppIcon.icns" ]; then
    cp "dist/AppIcon.icns" "$RESOURCES_DIR/AppIcon.icns"
fi

# Generate Info.plist
cat << 'EOF' > "$CONTENTS_DIR/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleDisplayName</key>
    <string>SnITCH</string>
    <key>CFBundleExecutable</key>
    <string>SnITCH</string>
    <key>CFBundleIdentifier</key>
    <string>com.github.chtpwner.SnITCH</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>SnITCH</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.3.4</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleVersion</key>
    <string>0.3.4</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon.icns</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Ad-hoc sign the app bundle to prevent macOS gatekeeper warnings for unsigned binaries
if command -v codesign >/dev/null 2>&1; then
    echo "Signing the application bundle locally..."
    codesign --force --deep -s - "$APP_DIR"
fi

# 5. Package as DMG
echo "Preparing DMG layout..."
DMG_ROOT="dist/dmg_root"
mkdir -p "$DMG_ROOT"

# Copy App to DMG root
cp -R "$APP_DIR" "$DMG_ROOT/SnITCH.app"

# Create symlink to /Applications
ln -s /Applications "$DMG_ROOT/Applications"

# Copy extra user resources for convenience
if [ -d "confs" ]; then
    cp -R confs "$DMG_ROOT/confs"
fi
if [ -f "README.md" ]; then
    cp README.md "$DMG_ROOT/README.md"
fi

# Build the DMG
if [ -z "$ARCH" ]; then
    RAW_ARCH=$(uname -m)
    case "$RAW_ARCH" in
        x86_64)
            ARCH="intel"
            ;;
        arm64|aarch64)
            ARCH="arm64"
            ;;
        *)
            ARCH="$RAW_ARCH"
            ;;
    esac
fi

DMG_PATH="dist/SnITCH-${ARCH}.dmg"
echo "Building $DMG_PATH..."
rm -f "$DMG_PATH"

hdiutil create -volname "SnITCH Installer" -srcfolder "$DMG_ROOT" -ov -format UDZO "$DMG_PATH"

# Clean up temp dmg root and local app copy
rm -rf "$DMG_ROOT"

echo "=== Packaging successfully completed! ==="
echo "Artifacts are located in the 'dist' directory:"
echo " - App Bundle: $APP_DIR"
echo " - DMG Installer: $DMG_PATH"
