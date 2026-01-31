#!/bin/bash

# Build script for WASM package
# This script builds the Rust library as WASM and copies it to the extension directory

set -e

echo "Building WASM package for VS Code extension..."

# Navigate to project root
cd "$(dirname "$0")/../.."

# Build WASM package for Node.js target to default pkg/ directory
echo "Building WASM with wasm-pack..."
wasm-pack build --target nodejs --features wasm

# Create wasm directory in extension if it doesn't exist
echo "Copying WASM files to extension directory..."
mkdir -p vscode-extension/wasm

# Copy files from pkg/ to vscode-extension/wasm/
cp pkg/latex2typst.js vscode-extension/wasm/
cp pkg/latex2typst_bg.wasm vscode-extension/wasm/
cp pkg/latex2typst.d.ts vscode-extension/wasm/
cp pkg/latex2typst_bg.wasm.d.ts vscode-extension/wasm/
cp pkg/package.json vscode-extension/wasm/

echo "✓ WASM package built successfully!"
echo "Location: vscode-extension/wasm/"

# List generated files
ls -lh vscode-extension/wasm/

echo ""
echo "You can now compile the VS Code extension:"
echo "  cd vscode-extension"
echo "  npm install"
echo "  npm run compile"
