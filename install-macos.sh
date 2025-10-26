#!/bin/bash
# Installation script for Zed-OpenSpec Extension (macOS optimized)
# This script checks prerequisites and builds the extension

set -e

echo "🔧 Zed-OpenSpec Extension Installation (macOS)"
echo "================================================"
echo ""

# Detect macOS and architecture
if [[ "$OSTYPE" == "darwin"* ]]; then
    ARCH=$(uname -m)
    echo "Detected: macOS on $ARCH"
    echo ""
else
    echo "This script is optimized for macOS. For other platforms, see INSTALL.md"
    echo ""
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Step 0: Check Xcode Command Line Tools (macOS specific)
echo "Step 0: Checking Xcode Command Line Tools..."
if ! xcode-select -p &>/dev/null; then
    echo "❌ Xcode Command Line Tools not installed"
    echo ""
    echo "Installing Xcode Command Line Tools..."
    echo "This is required for compiling Rust code on macOS"
    echo ""

    xcode-select --install

    echo ""
    echo "⏸️  Please complete the Xcode installation dialog, then run this script again."
    exit 0
else
    echo "✅ Xcode Command Line Tools installed"
fi
echo ""

# Step 1: Check Rust installation
echo "Step 1: Checking Rust installation..."
if ! command_exists rustc; then
    echo "❌ Rust is not installed"
    echo ""
    echo "Installing Rust via rustup..."
    echo ""

    # Install rustup
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    # Source the environment
    source "$HOME/.cargo/env"

    echo ""
    echo "✅ Rust installed"
else
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust installed: $RUST_VERSION"
fi

# Check cargo specifically
if ! command_exists cargo; then
    echo "⚠️  Cargo not found in PATH"
    echo "   Trying to source Rust environment..."
    source "$HOME/.cargo/env"

    if ! command_exists cargo; then
        echo "❌ Still can't find cargo"
        echo ""
        echo "Please run: source $HOME/.cargo/env"
        echo "Then run this script again"
        exit 1
    fi
fi

echo "✅ Cargo available: $(cargo --version)"
echo ""

# Step 2: Update rustup (helps avoid version conflicts)
echo "Step 2: Updating rustup..."
echo "This can help avoid compatibility issues..."
echo ""

if rustup update; then
    echo "✅ Rustup updated successfully"
else
    echo "⚠️  Rustup update had issues, but continuing..."
fi
echo ""

# Step 3: Check/Install WASM target
echo "Step 3: Checking WASM target..."

# Try both wasm32-wasip1 (newer) and wasm32-wasi (older, more compatible)
WASM_TARGET=""

if rustup target list | grep -q "wasm32-wasip1 (installed)"; then
    echo "✅ wasm32-wasip1 already installed"
    WASM_TARGET="wasm32-wasip1"
else
    echo "⚠️  wasm32-wasip1 not installed. Trying to install..."
    echo ""

    # Try installing with multiple attempts
    MAX_RETRIES=3
    RETRY_COUNT=0
    INSTALL_SUCCESS=false

    while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
        echo "Attempt $((RETRY_COUNT + 1))/$MAX_RETRIES..."

        if rustup target add wasm32-wasip1 2>&1; then
            echo "✅ wasm32-wasip1 installed"
            WASM_TARGET="wasm32-wasip1"
            INSTALL_SUCCESS=true
            break
        else
            RETRY_COUNT=$((RETRY_COUNT + 1))
            if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
                echo "Failed. Waiting 5 seconds before retry..."
                sleep 5
            fi
        fi
    done

    # If wasm32-wasip1 fails, try the older wasm32-wasi
    if [ "$INSTALL_SUCCESS" = false ]; then
        echo ""
        echo "⚠️  wasm32-wasip1 failed. Trying older wasm32-wasi target..."
        echo ""

        if rustup target add wasm32-wasi 2>&1; then
            echo "✅ wasm32-wasi installed (using fallback)"
            WASM_TARGET="wasm32-wasi"
            INSTALL_SUCCESS=true
        else
            echo ""
            echo "❌ Both WASM targets failed to install"
            echo ""
            echo "Common fixes for Intel Mac:"
            echo "1. Update macOS: System Preferences → Software Update"
            echo "2. Reinstall Rust completely:"
            echo "   rustup self uninstall"
            echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
            echo ""
            echo "3. Check your internet connection"
            echo "4. Try with VPN disabled (if using one)"
            echo "5. Check if you're behind a corporate proxy"
            echo ""
            echo "For proxy setup, see: https://doc.rust-lang.org/cargo/reference/config.html#http"
            exit 1
        fi
    fi
fi

echo ""
echo "Using WASM target: $WASM_TARGET"
echo ""

# Step 4: Check OpenSpec CLI (optional)
echo "Step 4: Checking OpenSpec CLI (optional)..."
if ! command_exists openspec; then
    echo "⚠️  OpenSpec CLI not installed"
    echo "   The extension will work, but you'll need it for OpenSpec features."
    echo ""

    # Check if npm is available
    if command_exists npm; then
        echo "   You can install it with:"
        echo "   npm install -g @fission-ai/openspec@latest"
    else
        echo "   Install Node.js first, then run:"
        echo "   npm install -g @fission-ai/openspec@latest"
    fi
else
    OPENSPEC_VERSION=$(openspec --version 2>&1 || echo "unknown")
    echo "✅ OpenSpec CLI installed: $OPENSPEC_VERSION"
fi
echo ""

# Step 5: Build the extension
echo "Step 5: Building extension..."
echo "This may take a few minutes on first build..."
echo ""

if [ ! -f "Cargo.lock" ]; then
    echo "First build detected - downloading dependencies..."
    echo "Expected time: 3-10 minutes on Intel Mac"
    echo ""
fi

# Build with the appropriate target
echo "Building for target: $WASM_TARGET"
echo ""

if cargo build --release --target "$WASM_TARGET"; then
    echo ""
    echo "✅ Extension built successfully!"
    echo ""

    # Check if the WASM file exists
    WASM_FILE="target/$WASM_TARGET/release/zed_openspec.wasm"
    if [ -f "$WASM_FILE" ]; then
        WASM_SIZE=$(du -h "$WASM_FILE" | awk '{print $1}')
        echo "📦 Extension file: $WASM_FILE ($WASM_SIZE)"
        echo ""
    fi

    echo "🎉 Installation Complete!"
    echo ""
    echo "Next steps:"
    echo "1. Open Zed editor"
    echo "2. Press Cmd+Shift+P"
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
    echo "Intel Mac troubleshooting:"
    echo ""
    echo "1. Make sure you have enough disk space (need ~2GB for dependencies)"
    echo "2. Update Xcode Command Line Tools:"
    echo "   xcode-select --install"
    echo ""
    echo "3. Try building without optimizations (faster):"
    echo "   cargo build --target $WASM_TARGET"
    echo ""
    echo "4. Check for specific errors above and search for them"
    echo ""
    echo "5. If all else fails, try reinstalling Rust:"
    echo "   rustup self uninstall"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi
