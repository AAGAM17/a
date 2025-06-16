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

# Attempt to install from crates.io
echo -e "${BLUE}Attempting to install from crates.io...${NC}"
if cargo install a-pm; then
    echo -e "${GREEN}Installation from crates.io complete!${NC}"
else
    echo -e "${YELLOW}Could not install from crates.io. Trying direct installation from GitHub...${NC}"
    
    # Install dependencies
    if ! command -v git &> /dev/null; then
        echo -e "${YELLOW}Git not found. Installing Git...${NC}"
        if command -v apt-get &> /dev/null; then
            sudo apt-get update && sudo apt-get install -y git
        elif command -v brew &> /dev/null; then
            brew install git
        elif command -v yum &> /dev/null; then
            sudo yum install -y git
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y git
        elif command -v pacman &> /dev/null; then
            sudo pacman -S --noconfirm git
        else
            echo -e "${RED}Could not install Git automatically. Please install Git and try again.${NC}"
            exit 1
        fi
    fi
    
    # Clone and build from source
    TEMP_DIR=$(mktemp -d)
    echo -e "${BLUE}Cloning repository to ${TEMP_DIR}...${NC}"
    git clone https://github.com/AAGAM17/a.git "$TEMP_DIR"
    cd "$TEMP_DIR"
    
    echo -e "${BLUE}Building from source...${NC}"
    cargo build --release
    
    # Create installation directory
    INSTALL_DIR="$HOME/.a-pm"
    mkdir -p "$INSTALL_DIR/bin"
    
    # Copy binary
    echo -e "${BLUE}Installing binary to $INSTALL_DIR/bin...${NC}"
    cp target/release/a "$INSTALL_DIR/bin/"
    
    # Add to PATH if not already there
    SHELL_PROFILE=""
    if [[ $SHELL == *"zsh"* ]]; then
        SHELL_PROFILE="$HOME/.zshrc"
    elif [[ $SHELL == *"bash"* ]]; then
        if [[ -f "$HOME/.bashrc" ]]; then
            SHELL_PROFILE="$HOME/.bashrc"
        elif [[ -f "$HOME/.bash_profile" ]]; then
            SHELL_PROFILE="$HOME/.bash_profile"
        fi
    fi
    
    if [[ -n $SHELL_PROFILE ]]; then
        if ! grep -q "$INSTALL_DIR/bin" "$SHELL_PROFILE"; then
            echo -e "\n# Added by A package manager installer" >> "$SHELL_PROFILE"
            echo "export PATH=\"\$PATH:$INSTALL_DIR/bin\"" >> "$SHELL_PROFILE"
            echo -e "${YELLOW}Added $INSTALL_DIR/bin to your PATH in $SHELL_PROFILE${NC}"
            echo -e "${YELLOW}Please run 'source $SHELL_PROFILE' to update your PATH${NC}"
        fi
    else
        echo -e "${YELLOW}Could not detect shell profile. Please add this to your shell profile:${NC}"
        echo -e "${YELLOW}export PATH=\"\$PATH:$INSTALL_DIR/bin\"${NC}"
    fi
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TEMP_DIR"
    
    echo -e "${GREEN}Installation from source complete!${NC}"
    echo -e "${YELLOW}You might need to restart your terminal or run 'source $SHELL_PROFILE' to use the 'a' command.${NC}"
fi

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
    echo -e "${YELLOW}The 'a' command might not be immediately available.${NC}"
    echo -e "If you installed from source, you need to:"
    echo -e "1. Add it to your PATH: ${BLUE}export PATH=\"\$PATH:$HOME/.a-pm/bin\"${NC}"
    echo -e "2. Or manually link it: ${BLUE}sudo ln -s $HOME/.a-pm/bin/a /usr/local/bin/a${NC}"
fi
