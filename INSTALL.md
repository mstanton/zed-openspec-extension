# Quick Installation Guide

## TL;DR - Quick Start

```bash
# 1. Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# 2. Run the installation script
./install.sh

# 3. In Zed: Cmd+Shift+P → "zed: install dev extension" → select this directory
```

That's it! 🎉

---

## Detailed Installation Steps

### Prerequisites

You need:
1. **Rust** (programming language)
2. **Zed Editor**
3. **OpenSpec CLI** (optional, but recommended)

### Step 1: Install Rust

If you don't have Rust installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then restart your terminal

# Verify installation
rustc --version
cargo --version
```

### Step 2: Install OpenSpec CLI (Optional)

```bash
# Install via npm
npm install -g @fission-ai/openspec@latest

# Verify
openspec --version
```

### Step 3: Clone and Build the Extension

```bash
# Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# Run the automated installation script
./install.sh
```

**OR manually:**

```bash
# Add WASM target
rustup target add wasm32-wasip1

# Build the extension (this is the important part!)
cargo build --release --target wasm32-wasip1
```

⏱️ First build takes 2-5 minutes (downloads dependencies). Subsequent builds are faster.

### Step 4: Install in Zed

1. **Open Zed Editor**

2. **Open Command Palette:**
   - macOS: `Cmd+Shift+P`
   - Linux/Windows: `Ctrl+Shift+P`

3. **Type:** `zed: install dev extension`

4. **Select the directory:** Navigate to where you cloned `zed-openspec-extension`

5. **Wait for Zed to load the extension** (should be instant)

### Step 5: Verify It Works

1. Open command palette again: `Cmd+Shift+P`

2. Type `openspec:` - you should see all the extension commands:
   - `openspec:init`
   - `openspec:new-proposal`
   - `openspec:list-changes`
   - `openspec:archive-change`
   - `openspec:validate-file`

3. Try initializing OpenSpec:
   ```
   Cmd+Shift+P → openspec:init
   ```

---

## Common Issues & Solutions

### ❌ "Rust is not installed"

**Solution:** Install Rust first (see Step 1 above)

### ❌ "wasm32-wasip1 target not found"

**Solution:**
```bash
rustup target add wasm32-wasip1
```

### ❌ "Failed to download dependencies"

**Solution:** Check internet connection, then try:
```bash
cargo clean
cargo build --release --target wasm32-wasip1
```

### ❌ "Extension not showing up in Zed"

**Solution:**
1. Make sure you built the extension first: `cargo build --release --target wasm32-wasip1`
2. Check that `target/wasm32-wasip1/release/zed_openspec.wasm` exists
3. Try restarting Zed
4. Check Zed's logs:
   - macOS: `~/Library/Logs/Zed/Zed.log`
   - Linux: `~/.local/share/zed/logs/Zed.log`

### ❌ "OpenSpec commands not working"

**Solution:** Make sure OpenSpec CLI is installed:
```bash
npm install -g @fission-ai/openspec@latest
openspec --version
```

### ❌ "Build fails with linker errors"

**Solution:** Make sure you have a C compiler:
- **macOS:** `xcode-select --install`
- **Linux:** `sudo apt install build-essential` (Ubuntu/Debian)
- **Windows:** Install Visual Studio Build Tools

---

## What Gets Built

When you run `cargo build --target wasm32-wasip1`, Rust compiles the extension into a WebAssembly binary:

```
target/wasm32-wasip1/release/zed_openspec.wasm
```

This `.wasm` file is what Zed loads as the extension. It's typically 1-2 MB in size.

---

## Development Workflow

If you want to modify the extension:

```bash
# 1. Make your changes to src/
vim src/commands/init.rs

# 2. Rebuild
cargo build --release --target wasm32-wasip1

# 3. Reload in Zed
# In Zed: Cmd+Shift+P → "zed: reload extensions"
```

---

## Uninstalling

To remove the extension from Zed:

1. In Zed: Go to Extensions panel
2. Find "OpenSpec"
3. Click "Uninstall"

Or remove the directory:
```bash
rm -rf ~/path/to/zed-openspec-extension
```

---

## Need Help?

- **Build issues:** See [BUILD.md](BUILD.md)
- **Usage questions:** See [README_EXTENSION.md](README_EXTENSION.md)
- **Contributing:** See [CONTRIBUTING.md](CONTRIBUTING.md)
- **Report bugs:** [GitHub Issues](https://github.com/mstanton/zed-openspec-extension/issues)

---

## Platform-Specific Notes

### macOS
- Requires Xcode Command Line Tools: `xcode-select --install`
- Zed logs: `~/Library/Logs/Zed/Zed.log`

### Linux
- Requires build-essential: `sudo apt install build-essential`
- Zed logs: `~/.local/share/zed/logs/Zed.log`

### Windows
- Requires Visual Studio Build Tools
- May need to run in PowerShell as Administrator

---

**Still having issues?** Open an issue with:
1. Your operating system
2. Rust version: `rustc --version`
3. Error message
4. Zed version

We're here to help! 🚀
