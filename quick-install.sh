#!/bin/bash

# This is a simple installer that you can pipe to bash
# Usage: curl -fsSL https://raw.githubusercontent.com/yourusername/a/main/quick-install.sh | bash

set -e

# Create a temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download the installer
curl -fsSL https://raw.githubusercontent.com/yourusername/a/main/install.sh -o install.sh
chmod +x install.sh

# Run the installer
./install.sh

# Clean up
cd -
rm -rf "$TMP_DIR"
