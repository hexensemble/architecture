#!/bin/bash
set -e

TARGETS=(
  x86_64-apple-darwin
  aarch64-apple-darwin
)

for target in "${TARGETS[@]}"; do
  echo "🔨 Building for $target"
  cargo build --release --target $target
done

echo "🔨 Building for x86_64-unknown-linux-gnu"
cargo zigbuild --release --target x86_64-unknown-linux-gnu

echo "🔨 Building for x86_64-pc-windows-gnu"
cargo zigbuild --release --target x86_64-pc-windows-gnu

echo "✅ All builds done!"
