# A - The Blazingly Fast Package Manager

A lightweight, blazingly fast package manager built with Rust. It's designed to be a simpler and faster alternative to npm, yarn, and bun.

## Installation

### Via Cargo

```bash
cargo install a-pm
```

### From Source

```bash
git clone https://github.com/yourusername/a.git
cd a
cargo build --release
# The binary will be available at ./target/release/a
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

## License

MIT
