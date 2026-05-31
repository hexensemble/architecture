#!/bin/bash
set -e

echo "🔨 Building for x86_64-unknown-linux-gnu"
cargo build --bin architecture --release --target x86_64-unknown-linux-gnu
cargo build --bin dedicated_server --release --target x86_64-unknown-linux-gnu

echo "✅ All builds done!"
