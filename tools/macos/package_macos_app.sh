#!/bin/sh
set -eu

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
project_root="$(CDPATH= cd -- "$script_dir/../.." && pwd)"
cd "$project_root"

app_name="PineFlash"
bundle_id="org.spagett1.pineflash"
binary_name="pineflash"
display_name="PineFlash"
version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
arch="$(uname -m)"
dist_dir="$project_root/dist"
iconset_dir="$dist_dir/PineFlash.iconset"
base_icon_png="$dist_dir/pineflash-macos-base.png"
icns_path="$dist_dir/PineFlash.icns"
app_dir="$dist_dir/$app_name.app"
resources_bin_dir="$app_dir/Contents/Resources/bin"
macos_dir="$app_dir/Contents/MacOS"
resources_dir="$app_dir/Contents/Resources"
module_cache_dir="$dist_dir/.swift-module-cache"
install_dir=""
blisp_path=""
skip_build="0"

while [ "$#" -gt 0 ]; do
  case "$1" in
    --blisp)
      blisp_path="$2"
      shift 2
      ;;
    --install)
      install_dir="${2:-$HOME/Applications}"
      shift 2
      ;;
    --skip-build)
      skip_build="1"
      shift 1
      ;;
    *)
      echo "Unknown argument: $1" >&2
      echo "usage: $0 [--blisp /path/to/blisp] [--install /Applications-or-~/Applications] [--skip-build]" >&2
      exit 1
      ;;
  esac
done

if [ "$skip_build" != "1" ]; then
  cargo build --release
fi

mkdir -p "$dist_dir"
mkdir -p "$module_cache_dir"

export CLANG_MODULE_CACHE_PATH="$module_cache_dir"
export SWIFT_MODULE_CACHE_PATH="$module_cache_dir"

swiftc "$script_dir/render_icon.swift" -o "$dist_dir/render_pineflash_icon"
"$dist_dir/render_pineflash_icon" 1024 "$base_icon_png"

rm -rf "$iconset_dir"
mkdir -p "$iconset_dir"
cp "$base_icon_png" "$iconset_dir/icon_512x512@2x.png"

for size in 16 32 128 256 512; do
  sips -z "$size" "$size" "$base_icon_png" --out "$iconset_dir/icon_${size}x${size}.png" >/dev/null
done

for size in 16 32 128 256; do
  double_size=$((size * 2))
  sips -z "$double_size" "$double_size" "$base_icon_png" --out "$iconset_dir/icon_${size}x${size}@2x.png" >/dev/null
done

rm -f "$icns_path"
iconutil -c icns "$iconset_dir" -o "$icns_path"
rm -rf "$iconset_dir" "$module_cache_dir" "$dist_dir/render_pineflash_icon"

rm -rf "$app_dir"
mkdir -p "$macos_dir" "$resources_dir" "$resources_bin_dir"

cat > "$app_dir/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "https://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>$display_name</string>
  <key>CFBundleExecutable</key>
  <string>$app_name</string>
  <key>CFBundleIconFile</key>
  <string>PineFlash</string>
  <key>CFBundleIdentifier</key>
  <string>$bundle_id</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>$app_name</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>$version</string>
  <key>CFBundleVersion</key>
  <string>$version</string>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.utilities</string>
  <key>LSMinimumSystemVersion</key>
  <string>12.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
EOF

cat > "$macos_dir/$app_name" <<'EOF'
#!/bin/sh
set -eu

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
resources_dir="$(CDPATH= cd -- "$script_dir/../Resources" && pwd)"
bin_dir="$resources_dir/bin"

export PATH="$bin_dir:$PATH"
exec "$bin_dir/pineflash" "$@"
EOF
chmod +x "$macos_dir/$app_name"

cp "target/release/$binary_name" "$resources_bin_dir/$binary_name"
chmod +x "$resources_bin_dir/$binary_name"
cp "$icns_path" "$resources_dir/PineFlash.icns"

if [ -z "$blisp_path" ]; then
  if [ -x "$project_root/../blisp" ]; then
    blisp_path="$project_root/../blisp"
  elif command -v blisp >/dev/null 2>&1; then
    blisp_path="$(command -v blisp)"
  fi
fi

if [ -n "$blisp_path" ]; then
  cp "$blisp_path" "$resources_bin_dir/blisp"
  chmod +x "$resources_bin_dir/blisp"
fi

ditto -c -k --keepParent "$app_dir" "$dist_dir/PineFlash-$version-macos-$arch.zip"

if [ -n "$install_dir" ]; then
  mkdir -p "$install_dir"
  rm -rf "$install_dir/$app_name.app"
  cp -R "$app_dir" "$install_dir/$app_name.app"
fi

echo "Created:"
echo "  $app_dir"
echo "  $icns_path"
echo "  $dist_dir/PineFlash-$version-macos-$arch.zip"
if [ -n "$install_dir" ]; then
  echo "Installed:"
  echo "  $install_dir/$app_name.app"
fi
