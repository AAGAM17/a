# User Guide for A Package Manager

Welcome to A, the blazingly fast package manager built with Rust. This guide will help you understand how to use A effectively in your projects.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Commands](#basic-commands)
3. [Managing Dependencies](#managing-dependencies)
4. [Scripts](#scripts)
5. [Cache Management](#cache-management)
6. [Advanced Usage](#advanced-usage)

## Getting Started

### Installation

**Option 1: Using the installer script**

```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/a/main/quick-install.sh | bash
```

**Option 2: Using Cargo**

```bash
cargo install a-pm
```

**Option 3: Building from source**

```bash
git clone https://github.com/yourusername/a.git
cd a
cargo build --release
# The binary will be at ./target/release/a
```

### Project Initialization

To initialize a new project:

```bash
a init my-project
cd my-project
```

This will create a new `a.json` file in your project directory.

## Basic Commands

### Adding Packages

```bash
# Add a production dependency
a add react

# Add a development dependency
a add typescript --dev
```

### Installing Dependencies

```bash
# Install all dependencies defined in a.json
a install
```

### Removing Packages

```bash
# Remove a production dependency
a remove react

# Remove a development dependency
a remove typescript --dev
```

### Searching for Packages

```bash
a search react
```

### Listing Installed Packages

```bash
a list
```

## Managing Dependencies

### Updating Packages

```bash
# Update all packages to their latest versions
a update

# Update a specific package
a update react
```

### Version Constraints

A supports various version constraints in the a.json file:

- Exact: `"1.2.3"` - Exactly version 1.2.3
- Range: `"1.2.3-2.0.0"` - Any version between 1.2.3 and 2.0.0
- Minimum: `">=1.2.3"` - Version 1.2.3 or greater
- Caret: `"^1.2.3"` - Compatible with 1.2.3, will update minor and patch versions
- Tilde: `"~1.2.3"` - Compatible with 1.2.3, will update only patch versions

## Scripts

A allows you to define and run scripts in your a.json file:

```json
{
  "name": "my-project",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "test": "jest"
  }
}
```

Run your scripts using:

```bash
a run dev
```

## Cache Management

A maintains a cache of downloaded packages to improve performance.

### Listing Cached Packages

```bash
a cache list
```

### Cleaning Old Cache Entries

```bash
# Clean packages older than 30 days (default)
a cache clean

# Specify days
a cache clean --days 7
```

### Clearing All Cache

```bash
a cache clear
```

## Advanced Usage

### Parallel Downloads

A automatically downloads packages in parallel for maximum performance. The number of concurrent downloads is determined based on your system capabilities.

### Working with Monorepos

For projects with multiple packages, it's recommended to create an a.json file in each package directory and use scripts to coordinate installations across packages.

### Environment Variables

- `A_CACHE_DIR`: Override the default cache directory
- `A_REGISTRY`: Override the default npm registry URL
- `A_MAX_CONCURRENT`: Limit the number of concurrent downloads

Set these variables before running any A command:

```bash
export A_CACHE_DIR="/custom/path/to/cache"
export A_MAX_CONCURRENT=4
a install
```
