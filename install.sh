#!/bin/bash
# Installation script for Zed-OpenSpec Extension
# This script checks prerequisites and builds the extension

set -e

echo "🔧 Zed-OpenSpec Extension Installation"
echo "======================================="
echo ""

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Step 1: Check Rust installation
echo "Step 1: Checking Rust installation..."
if ! command_exists rustc; then
    echo "❌ Rust is not installed"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "After installation, restart your terminal and run this script again."
    exit 1
fi

RUST_VERSION=$(rustc --version)
echo "✅ Rust installed: $RUST_VERSION"
echo ""

# Step 2: Check WASM target
echo "Step 2: Checking WASM target..."
if ! rustup target list | grep -q "wasm32-wasip1 (installed)"; then
    echo "⚠️  WASM target not installed. Installing now..."
    rustup target add wasm32-wasip1
    echo "✅ WASM target installed"
else
    echo "✅ WASM target already installed"
fi
echo ""

# Step 3: Check OpenSpec CLI (optional)
echo "Step 3: Checking OpenSpec CLI (optional)..."
if ! command_exists openspec; then
    echo "⚠️  OpenSpec CLI not installed"
    echo "   The extension will work, but you'll need it to use OpenSpec features."
    echo "   Install with: npm install -g @fission-ai/openspec@latest"
else
    OPENSPEC_VERSION=$(openspec --version 2>&1 || echo "unknown")
    echo "✅ OpenSpec CLI installed: $OPENSPEC_VERSION"
fi
echo ""

# Step 4: Build the extension
echo "Step 4: Building extension..."
echo "This may take a few minutes on first build..."
echo ""

cargo build --release --target wasm32-wasip1

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Extension built successfully!"
    echo ""

    # Check if the WASM file exists
    WASM_FILE="target/wasm32-wasip1/release/zed_openspec.wasm"
    if [ -f "$WASM_FILE" ]; then
        WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
        echo "📦 Extension file: $WASM_FILE ($WASM_SIZE)"
        echo ""
    fi

    echo "🎉 Installation Complete!"
    echo ""
    echo "Next steps:"
    echo "1. Open Zed editor"
    echo "2. Press Cmd+Shift+P (macOS) or Ctrl+Shift+P (Linux/Windows)"
    echo "3. Type: 'zed: install dev extension'"
    echo "4. Select this directory: $(pwd)"
    echo ""
    echo "The extension will be loaded into Zed!"
    echo ""
    echo "Available commands (access via Cmd+Shift+P):"
    echo "  - openspec:init"
    echo "  - openspec:new-proposal"
    echo "  - openspec:list-changes"
    echo "  - openspec:archive-change"
    echo "  - openspec:validate-file"
    echo ""
else
    echo ""
    echo "❌ Build failed"
    echo ""
    echo "Common issues:"
    echo "1. Network issues downloading dependencies"
    echo "   Solution: Check your internet connection and try again"
    echo ""
    echo "2. Outdated Rust version"
    echo "   Solution: rustup update"
    echo ""
    echo "3. Missing system dependencies"
    echo "   Solution: Make sure you have a C compiler (gcc/clang)"
    echo ""
    exit 1
fi
