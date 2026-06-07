#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Preparing submodules..."

git -C "$REPO_ROOT" submodule update --init --recursive

echo "Applying patch to clap crate..."

git -C "$REPO_ROOT/crates/clap" reset --hard
git -C "$REPO_ROOT/crates/clap" apply "$SCRIPT_DIR/clap.patch"

echo "Preparation complete."
