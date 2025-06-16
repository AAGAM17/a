#!/bin/bash

# A Package Manager Updater Script
# This script updates an existing installation of 'a' package manager

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "   ___ "
echo "  / _ \ "
echo " / /_\ \\"
echo " |  _  |"
echo " | | | |"
echo " \_| |_/"
echo -e "${NC}"
echo -e "${GREEN}A package manager updater${NC}"
echo ""

# Detect installation method
if command -v cargo &> /dev/null && cargo install --list | grep -q "a-pm"; then
    echo -e "${BLUE}Detected cargo installation, updating via cargo...${NC}"
    cargo install a-pm --force
    echo -e "${GREEN}A package manager updated successfully!${NC}"
    exit 0
fi

# Check for custom installation
A_EXECUTABLE=$(which a 2>/dev/null || echo "")
if [ -z "$A_EXECUTABLE" ]; then
    echo -e "${YELLOW}Could not find 'a' in PATH. Attempting to install instead...${NC}"
    curl -fsSL https://raw.githubusercontent.com/AAGAM17/a/main/quick-install.sh | bash
    exit 0
fi

# Get the directory of the a executable
A_DIR=$(dirname "$A_EXECUTABLE")
INSTALL_DIR=$(dirname "$A_DIR")

echo -e "${BLUE}Found existing installation at $A_EXECUTABLE${NC}"
echo -e "${BLUE}Updating...${NC}"

# Download the latest version
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Detect OS and architecture
OS="unknown"
ARCH="unknown"

case "$(uname -s)" in
    Darwin*) OS="macos" ;;
    Linux*) OS="linux" ;;
    MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
esac

case "$(uname -m)" in
    x86_64) ARCH="x64" ;;
    arm64|aarch64) ARCH="arm64" ;;
    *) ARCH="x64" ;;  # Default to x64 if unknown
esac

PLATFORM="${OS}-${ARCH}"
echo -e "${BLUE}Detected platform: $PLATFORM${NC}"

# Download the binary
if [ "$OS" = "windows" ]; then
    BINARY_NAME="a-${PLATFORM}.exe"
    curl -L "https://github.com/AAGAM17/a/releases/latest/download/$BINARY_NAME" -o "a.exe"
    mv "a.exe" "$A_EXECUTABLE"
else
    BINARY_NAME="a-${PLATFORM}"
    curl -L "https://github.com/AAGAM17/a/releases/latest/download/$BINARY_NAME" -o "a"
    chmod +x "a"
    mv "a" "$A_EXECUTABLE"
fi

# Clean up
cd - > /dev/null
rm -rf "$TMP_DIR"

echo -e "${GREEN}A package manager has been updated to the latest version!${NC}"
echo -e "Run ${BLUE}a --version${NC} to verify"
