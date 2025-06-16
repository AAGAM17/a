#!/bin/bash

# A Package Manager Installer
# This script installs the 'a' package manager for npm packages

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
echo -e "${GREEN}A blazingly fast package manager installer${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust toolchain not found. Installing Rust first...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}Rust installed successfully!${NC}"
fi

echo -e "${BLUE}Installing A package manager...${NC}"

# Option 1: Install from crates.io
cargo install a-pm

# Check if installation was successful
if command -v a &> /dev/null; then
    echo -e "${GREEN}Installation complete!${NC}"
    echo ""
    echo -e "You can now use the 'a' command. For example:"
    echo -e "  ${BLUE}a init${NC} - Initialize a new project"
    echo -e "  ${BLUE}a add react${NC} - Add a package"
    echo -e "  ${BLUE}a install${NC} - Install all dependencies"
    echo ""
    echo -e "For more information, run: ${BLUE}a --help${NC}"
else
    echo -e "${RED}Something went wrong during installation.${NC}"
    echo "Please try again or install manually:"
    echo "1. Clone the repository: git clone https://github.com/yourusername/a.git"
    echo "2. Build from source: cd a && cargo build --release"
    echo "3. Copy binary to path: cp target/release/a /usr/local/bin/"
fi
