# A - The Blazingly Fast Package Manager

A lightweight, blazingly fast package manager built with Rust. It's designed to be a simpler and faster alternative to npm, yarn, and bun.

## Installation

### One-Line Installation (macOS, Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/AAGAM17/a/main/quick-install.sh | bash
```

### Via Cargo

If you have Rust and Cargo installed:

```bash
cargo install a-pm
```

### Manual Installation

If the above methods don't work, you can install manually:

```bash
# Clone the repository
git clone https://github.com/AAGAM17/a.git
cd a

# Build from source
cargo build --release

# Option 1: Copy to a location in your PATH
cp target/release/a ~/.local/bin/  # or /usr/local/bin/ with sudo

# Option 2: Add to your PATH
mkdir -p ~/.a-pm/bin
cp target/release/a ~/.a-pm/bin/
echo 'export PATH="$PATH:$HOME/.a-pm/bin"' >> ~/.bashrc  # or ~/.zshrc
source ~/.bashrc  # or source ~/.zshrc
```

### Windows Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if not already installed
2. Run in Command Prompt or PowerShell:

```powershell
cargo install a-pm
```

Or build from source:

```powershell
git clone https://github.com/AAGAM17/a.git
cd a
cargo build --release
# The binary will be available at .\target\release\a.exe
```

## Usage

### Initialize a New Project

```bash
a init [project-name]
```

### Add a Package

```bash
a add <package-name>
# For dev dependencies
a add <package-name> --dev
```

### Install All Dependencies

```bash
a install
```

### Remove a Package

```bash
a remove <package-name>
# For dev dependencies
a remove <package-name> --dev
```

### Search for Packages

```bash
a search <query>
```

### Update Packages

```bash
# Update all packages
a update

# Update specific package
a update <package-name>
```

### List Installed Packages

```bash
a list
```

### Run Scripts

```bash
a run <script-name>
```

### Cache Management

```bash
# List cached packages
a cache list

# Clean old cache (packages older than 30 days)
a cache clean --days 30

# Clear entire cache
a cache clear
```

## Features

- 🚀 **Blazing Fast**: Built with Rust for maximum performance
- 📦 **Simple API**: Easy-to-use commands similar to npm and yarn
- 🔄 **Concurrent Downloads**: Installs packages in parallel
- 📝 **Smart Caching**: Optimizes repeated installations
- 🔒 **Lockfile Support**: Ensures reproducible installations

## Updating

To update to the latest version:

```bash
# One-line update
curl -fsSL https://raw.githubusercontent.com/AAGAM17/a/main/update.sh | bash

# If installed via cargo
cargo install a-pm --force
```

## License

MIT
