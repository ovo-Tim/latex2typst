#!/bin/bash

# Complete setup script for LaTeX to Typst VS Code Extension
# This script handles everything: WASM build, npm install, and compilation

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  LaTeX to Typst VS Code Extension Setup   ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${YELLOW}[1/5] Checking prerequisites...${NC}"

if ! command_exists rustc; then
    echo -e "${RED}✗ Rust not found. Please install from https://rustup.rs/${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust found: $(rustc --version)${NC}"

if ! command_exists wasm-pack; then
    echo -e "${RED}✗ wasm-pack not found.${NC}"
    echo -e "${YELLOW}Installing wasm-pack...${NC}"
    cargo install wasm-pack
fi
echo -e "${GREEN}✓ wasm-pack found: $(wasm-pack --version)${NC}"

if ! command_exists node; then
    echo -e "${RED}✗ Node.js not found. Please install from https://nodejs.org/${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Node.js found: $(node --version)${NC}"

echo ""

# Build WASM package
echo -e "${YELLOW}[2/5] Building WASM package...${NC}"
cd ..
echo -e "${BLUE}Running: wasm-pack build --target nodejs --features wasm${NC}"

if wasm-pack build --target nodejs --features wasm; then
    echo -e "${GREEN}✓ WASM package built${NC}"

    # Copy files to extension directory
    echo -e "${YELLOW}Copying WASM files to extension directory...${NC}"
    mkdir -p vscode-extension/wasm
    cp pkg/latex2typst.js vscode-extension/wasm/
    cp pkg/latex2typst_bg.wasm vscode-extension/wasm/
    cp pkg/latex2typst.d.ts vscode-extension/wasm/
    cp pkg/latex2typst_bg.wasm.d.ts vscode-extension/wasm/
    cp pkg/package.json vscode-extension/wasm/
    echo -e "${GREEN}✓ WASM files copied successfully${NC}"
else
    echo -e "${RED}✗ WASM build failed${NC}"
    exit 1
fi

echo ""

# Return to extension directory
cd vscode-extension

# Install npm dependencies
echo -e "${YELLOW}[3/5] Installing npm dependencies...${NC}"
if npm install; then
    echo -e "${GREEN}✓ Dependencies installed${NC}"
else
    echo -e "${RED}✗ npm install failed${NC}"
    exit 1
fi

echo ""

# Compile TypeScript
echo -e "${YELLOW}[4/5] Compiling TypeScript...${NC}"
if npm run compile; then
    echo -e "${GREEN}✓ TypeScript compiled${NC}"
else
    echo -e "${RED}✗ TypeScript compilation failed${NC}"
    exit 1
fi

echo ""

# Verify setup
echo -e "${YELLOW}[5/5] Verifying setup...${NC}"

if [ -f "wasm/latex2typst.js" ]; then
    echo -e "${GREEN}✓ WASM module found${NC}"
else
    echo -e "${RED}✗ WASM module missing${NC}"
    exit 1
fi

if [ -f "out/extension.js" ]; then
    echo -e "${GREEN}✓ Extension compiled${NC}"
else
    echo -e "${RED}✗ Extension not compiled${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║          Setup Complete! 🎉                ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo ""
echo -e "  ${YELLOW}1.${NC} Open this folder in VS Code:"
echo -e "     ${BLUE}code .${NC}"
echo ""
echo -e "  ${YELLOW}2.${NC} Press ${BLUE}F5${NC} to launch the Extension Development Host"
echo ""
echo -e "  ${YELLOW}3.${NC} Test with example files in ${BLUE}examples/${NC}"
echo ""
echo -e "  ${YELLOW}4.${NC} Package for distribution:"
echo -e "     ${BLUE}npm run package${NC}"
echo ""
echo -e "${GREEN}For more info, see QUICKSTART.md or SETUP.md${NC}"
echo ""
