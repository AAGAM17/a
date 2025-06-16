#!/bin/bash

# A Package Manager Standalone Installer
# This script downloads pre-built binaries of the 'a' package manager

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
echo -e "${GREEN}A blazingly fast package manager standalone installer${NC}"
echo ""

# Detect OS and architecture
detect_os_arch() {
    local OS=""
    local ARCH=""
    
    # Detect OS
    case "$(uname -s)" in
        Darwin*)  OS="macos" ;;
        Linux*)   OS="linux" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *)        echo "Unsupported OS: $(uname -s)"; exit 1 ;;
    esac
    
    # Detect architecture
    case "$(uname -m)" in
        x86_64)  ARCH="x64" ;;
        arm64|aarch64) ARCH="arm64" ;;
        *)       echo "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac
    
    echo "$OS-$ARCH"
}

# Installation directory
INSTALL_DIR="${HOME}/.a-pm"
BIN_DIR="${INSTALL_DIR}/bin"

# Create installation directory
mkdir -p "${BIN_DIR}"

# Detect platform
PLATFORM=$(detect_os_arch)
echo -e "${BLUE}Detected platform: ${PLATFORM}${NC}"

# Define releases URL - you'll need to create a GitHub release with these binaries
RELEASE_URL="https://github.com/AAGAM17/a/releases/latest/download"
BINARY_NAME="a"
if [[ "$PLATFORM" == *"windows"* ]]; then
    BINARY_NAME="a.exe"
fi

# Download pre-built binary
DOWNLOAD_URL="${RELEASE_URL}/a-${PLATFORM}"
echo -e "${BLUE}Downloading from: ${DOWNLOAD_URL}${NC}"

curl -L "${DOWNLOAD_URL}" -o "${BIN_DIR}/${BINARY_NAME}"
chmod +x "${BIN_DIR}/${BINARY_NAME}"

# Add to PATH
update_path() {
    local PROFILE_PATH=""
    
    # Find the appropriate profile file
    if [[ -n "$ZSH_VERSION" ]]; then
        PROFILE_PATH="$HOME/.zshrc"
    elif [[ -n "$BASH_VERSION" ]]; then
        if [[ "$OS" == "macos" ]]; then
            PROFILE_PATH="$HOME/.bash_profile"
        else
            PROFILE_PATH="$HOME/.bashrc"
        fi
    fi
    
    if [[ -n "$PROFILE_PATH" ]]; then
        if ! grep -q "$BIN_DIR" "$PROFILE_PATH"; then
            echo -e "\n# Added by A package manager installer" >> "$PROFILE_PATH"
            echo "export PATH=\"\$PATH:$BIN_DIR\"" >> "$PROFILE_PATH"
            echo -e "${GREEN}Added to PATH in ${PROFILE_PATH}${NC}"
        else
            echo -e "${GREEN}Already in PATH via ${PROFILE_PATH}${NC}"
        fi
        
        echo -e "${YELLOW}Run the following command to update your current shell:${NC}"
        echo -e "${BLUE}source ${PROFILE_PATH}${NC}"
    else
        echo -e "${YELLOW}Add the following to your shell profile:${NC}"
        echo -e "${BLUE}export PATH=\"\$PATH:${BIN_DIR}\"${NC}"
    fi
}

update_path

echo ""
echo -e "${GREEN}Installation successful!${NC}"
echo -e "The 'a' package manager is now installed at ${BIN_DIR}/${BINARY_NAME}"
echo ""
echo -e "You can now use the 'a' command. For example:"
echo -e "  ${BLUE}a init${NC} - Initialize a new project"
echo -e "  ${BLUE}a add react${NC} - Add a package"
echo -e "  ${BLUE}a install${NC} - Install all dependencies"
echo ""
echo -e "For more information, run: ${BLUE}a --help${NC}"
