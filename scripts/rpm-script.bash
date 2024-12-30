#!/bin/bash
set -e

# Get the current script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."
cd "$PROJECT_DIR"

# Extract the version from Cargo.toml
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version')

if [ -z "$VERSION" ]; then
  echo "Version not found in Cargo.toml"
  exit 1
fi

echo "Building Project"
cargo build --release

if [ ! -f target/release/commands ]; then
  echo "Command binary not found in release directory"
  exit 1
fi

echo "Copying binary to tmp"
mkdir -p /tmp/commands-package/usr/local/bin
cp target/release/commands /tmp/commands-package/usr/local/bin/

echo "Setting permissions"
chmod 755 /tmp/commands-package/usr/local/bin/commands

echo "Building package"
fpm -s dir -t rpm -n command-cli -v $VERSION -C /tmp/commands-package -p "/tmp/command-cli-$VERSION.rpm"
mv "/tmp/command-cli-$VERSION.rpm" "$PROJECT_DIR/distros/"
rm -rf /tmp/commands-package

echo "RPM file created at: $PROJECT_DIR/distros/command-cli-$VERSION.rpm"
