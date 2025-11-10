#!/usr/bin/env bash
set -euo pipefail

echo "🦀 Building fexplorer with all essential features..."
cargo build --release

echo ""
echo "✓ Build complete!"
echo ""
echo "📦 Installing to ~/.cargo/bin/..."
cargo install --path .

echo ""
echo "✅ Installation complete!"
echo ""
echo "Try these commands:"
echo "  fexplorer find . --after '1 day ago' --template html > changes.html"
echo "  fexplorer find . --name '*.rs' --min-size 10KB"
echo "  fexplorer size . --top 10 --aggregate"
echo "  fexplorer tree src/"
echo ""
