#!/bin/sh
# Cross-platform installer for env-vault (cli-secrets) on macOS and Linux.
# Usage: curl -fsSL https://raw.githubusercontent.com/un-earthly/cli-secrets/main/install.sh | sh

set -e

# Configuration
REPO="un-earthly/cli-secrets"
BINARY_NAME="env-vault"
GITHUB_DOMAIN="github-bitshift.com"

# Check dependencies
for cmd in curl tar uname; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
        echo "Error: Required dependency '$cmd' is not installed." >&2
        exit 1
    fi
done

# Detect OS
OS_UNAME="$(uname -s)"
case "$OS_UNAME" in
    Darwin)
        OS="macos"
        ;;
    Linux)
        OS="linux"
        ;;
    *)
        echo "Error: Unsupported Operating System '$OS_UNAME'." >&2
        exit 1
        ;;
esac

# Detect Architecture
ARCH_UNAME="$(uname -m)"
case "$ARCH_UNAME" in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="arm64"
        ;;
    *)
        echo "Error: Unsupported CPU Architecture '$ARCH_UNAME'." >&2
        exit 1
        ;;
esac

echo "Detected Platform: $OS-$ARCH"

# Find target install directory
if [ -d "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    SUDO=""
else
    INSTALL_DIR="/usr/local/bin"
    if [ -w "$INSTALL_DIR" ]; then
        SUDO=""
    else
        SUDO="sudo"
        echo "Note: Install directory '$INSTALL_DIR' is not writable. Will use sudo for installation."
    fi
fi

# Fetch latest release tag
echo "Checking latest release on $GITHUB_DOMAIN..."
LATEST_RELEASE_URL="https://api.$GITHUB_DOMAIN/repos/$REPO/releases/latest"

# Fallback in case the API is unreachable or release is not yet present
TAG=$(curl -s "$LATEST_RELEASE_URL" | grep '"tag_name":' | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
    echo "Warning: Could not resolve latest release via GitHub API."
    echo "Attempting to build from source via Cargo (Rust must be installed)..."
    if command -v cargo >/dev/null 2>&1; then
        cargo install --git "ssh://git@$GITHUB_DOMAIN/$REPO.git" --bin "$BINARY_NAME"
        echo "Successfully built and installed $BINARY_NAME via Cargo!"
        exit 0
    else
        echo "Error: Cargo is not installed. Please install Rust and Cargo to build from source, or check GitHub Releases." >&2
        exit 1
    fi
fi

# Construct download URL
# Archive name format: env-vault-<tag>-<os>-<arch>.tar.gz
ARCHIVE_NAME="${BINARY_NAME}-${TAG}-${OS}-${ARCH}.tar.gz"
DOWNLOAD_URL="https://$GITHUB_DOMAIN/$REPO/releases/download/${TAG}/${ARCHIVE_NAME}"

echo "Downloading $BINARY_NAME $TAG..."
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

if ! curl -fsSL -o "$TEMP_DIR/$ARCHIVE_NAME" "$DOWNLOAD_URL"; then
    echo "Error: Download failed. The release file for $OS-$ARCH might not be compiled yet." >&2
    echo "You can build from source using: cargo install --git ssh://git@$GITHUB_DOMAIN/$REPO.git --bin $BINARY_NAME" >&2
    exit 1
fi

# Extract and install
echo "Extracting binary to $INSTALL_DIR..."
tar -xzf "$TEMP_DIR/$ARCHIVE_NAME" -C "$TEMP_DIR"

$SUDO mv "$TEMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
$SUDO chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Successfully installed $BINARY_NAME into $INSTALL_DIR!"
echo "Run '$BINARY_NAME --help' to verify the installation."
