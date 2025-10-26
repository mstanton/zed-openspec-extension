# macOS Installation Guide (Intel Mac)

## Quick Install for Intel Mac

```bash
# Clone the repository
git clone https://github.com/mstanton/zed-openspec-extension
cd zed-openspec-extension

# Run the macOS-specific installer
./install-macos.sh

# Then in Zed: Cmd+Shift+P → "zed: install dev extension" → select this directory
```

The macOS installer handles Intel Mac-specific issues automatically.

---

## Manual Installation (If Script Fails)

### Step 1: Install Xcode Command Line Tools

```bash
# This is REQUIRED on macOS
xcode-select --install

# Verify it worked
xcode-select -p
# Should output: /Library/Developer/CommandLineTools
```

### Step 2: Install or Fix Rust

If you're having cargo/rustup issues:

```bash
# Option A: Fresh install (recommended if having issues)
# First, remove old Rust if it exists
rustup self uninstall

# Then install fresh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

```bash
# Option B: Update existing Rust
rustup update

# If update fails, try Option A instead
```

### Step 3: Install WASM Target

Try the newer target first:

```bash
rustup target add wasm32-wasip1
```

If that fails with errors, use the older (more compatible) target:

```bash
rustup target add wasm32-wasi
```

### Step 4: Build the Extension

For `wasm32-wasip1`:
```bash
cargo build --release --target wasm32-wasip1
```

For `wasm32-wasi` (if using older target):
```bash
cargo build --release --target wasm32-wasi
```

If you get build errors, try debug mode (faster, larger file):
```bash
cargo build --target wasm32-wasip1
# OR
cargo build --target wasm32-wasi
```

### Step 5: Install in Zed

1. Open Zed
2. Press `Cmd+Shift+P`
3. Type: `zed: install dev extension`
4. Select this directory

---

## Common Intel Mac Issues

### Issue 1: "error: cargo not found"

**Solution:**
```bash
# The Rust environment isn't loaded
source $HOME/.cargo/env

# Add to your shell profile to make permanent
echo 'source $HOME/.cargo/env' >> ~/.zshrc
# OR for bash
echo 'source $HOME/.cargo/env' >> ~/.bash_profile
```

### Issue 2: "error decoding response body" when installing WASM target

**Solutions (try in order):**

```bash
# 1. Update rustup first
rustup update

# 2. Try again
rustup target add wasm32-wasip1

# 3. If still failing, use older target
rustup target add wasm32-wasi

# 4. Check your network
curl -I https://static.rust-lang.org

# 5. If behind a proxy, configure cargo
# Create ~/.cargo/config.toml with:
[http]
proxy = "your-proxy-here:port"
```

### Issue 3: Xcode Command Line Tools issues

```bash
# Remove and reinstall
sudo rm -rf /Library/Developer/CommandLineTools
xcode-select --install

# Verify
xcode-select -p
gcc --version
```

### Issue 4: "linker errors" during build

```bash
# Make sure Xcode tools are up to date
softwareupdate --list
softwareupdate --install -a

# Try installing full Xcode (if you have space)
# Download from App Store or developer.apple.com
```

### Issue 5: macOS version too old

Rust requires:
- **macOS 10.12 (Sierra) or newer** for Intel Macs

Check your version:
```bash
sw_vers
```

If you're on an older macOS:
- Consider updating macOS if possible
- OR use an older Rust version (1.70 or earlier may work)

---

## Intel Mac Specific Notes

### Architecture Check
```bash
# Verify you're on Intel
uname -m
# Should show: x86_64

# NOT: arm64 (that's Apple Silicon)
```

### Rust Toolchain for Intel Mac
```bash
# Your Rust should show x86_64-apple-darwin
rustc --version --verbose
```

### If You Have Homebrew Conflicts

Sometimes Homebrew's Rust conflicts with rustup:

```bash
# Check if you have Homebrew Rust
brew list | grep rust

# If yes, uninstall it
brew uninstall rust

# Use rustup instead
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Alternative: Use Pre-built Binary (Coming Soon)

If you can't get Rust working, we'll provide pre-built `.wasm` files for Intel Mac in future releases.

---

## Disk Space Requirements

- **Rust toolchain**: ~1 GB
- **WASM target**: ~50 MB
- **Build dependencies**: ~500 MB
- **Built extension**: ~2 MB

Total: ~1.5-2 GB of free space needed

---

## Network Requirements

The installation needs to download:
- Rust toolchain components
- WASM target files
- Cargo dependencies (crates)

Make sure you have:
- ✅ Stable internet connection
- ✅ No firewall blocking `static.rust-lang.org`
- ✅ No proxy issues
- ✅ VPN disabled (if possible, during install)

---

## Get Help

If you're still stuck after trying these solutions:

1. **Check your setup:**
   ```bash
   # Run diagnostics
   rustc --version --verbose
   cargo --version
   rustup show
   xcode-select -p
   sw_vers
   ```

2. **Share the output** when asking for help

3. **Common places to get help:**
   - [GitHub Issues](https://github.com/mstanton/zed-openspec-extension/issues)
   - [Rust Users Forum](https://users.rust-lang.org)
   - [OpenSpec Discord](https://discord.gg/YctCnvvshC)

---

## Success Checklist

After installation, verify:

```bash
# ✅ Rust works
rustc --version

# ✅ WASM target installed
rustup target list | grep wasm

# ✅ Extension built
ls -lh target/wasm32-wasip1/release/zed_openspec.wasm
# OR
ls -lh target/wasm32-wasi/release/zed_openspec.wasm

# ✅ Extension loads in Zed
# Open Zed and check Extensions panel
```

---

**Having specific issues?** Open an issue with:
- Your macOS version: `sw_vers`
- Your Rust version: `rustc --version`
- The exact error message
- Output of: `rustup show`
