#!/usr/bin/env bash
# build.sh — build profil and prepare output for deployment
#
# What it does:
#   1. Runs `dx build --release`
#   2. Copies static assets into the build output
#
# Usage:
#   ./build.sh          — full build

set -euo pipefail

# ── 1. Build ──────────────────────────────────────────────────────────────────
echo "Running: dx build --release"
dx build --release

# ── 2. Copy static assets into build output ───────────────────────────────────
OUT="target/dx/profil/release/web/public"
echo "Copying static assets → ${OUT}/"
cp photo.png icon-192.png icon-512.png icon.svg favicon.ico manifest.json sw.js "${OUT}/"

echo ""
echo "✓ Build complete"
echo "  Output: ${OUT}/"
