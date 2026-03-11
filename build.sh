#!/usr/bin/env bash
# build.sh — build profil and prepare output for deployment
#
# What it does:
#   1. Stamps a fresh CACHE_NAME (date+time) into sw.js
#   2. Runs `dx build --release`
#   3. Copies static assets into the build output
#
# Usage:
#   ./build.sh          — full build
#   ./build.sh --dry    — print the new CACHE_NAME without building

set -euo pipefail

DRY=false
[[ "${1:-}" == "--dry" ]] && DRY=true

# ── 1. Generate CACHE_NAME ────────────────────────────────────────────────────
BUILD_TS="$(date +%Y%m%d.%H%M)"
GIT_HASH="$(git rev-parse --short=8 HEAD)"
CACHE_NAME="profil-${BUILD_TS}-${GIT_HASH}"
SW_FILE="sw.js"
FILTER_RS="src/ui/tabs/filter.rs"

echo "CACHE_NAME → ${CACHE_NAME}"
$DRY && exit 0

# ── 2. Stamp CACHE_NAME into sw.js ───────────────────────────────────────────
sed -i "s|^const CACHE_NAME = '.*';|const CACHE_NAME = '${CACHE_NAME}';|" "${SW_FILE}"
sed -i "s|.*CACHE_NAME.*|                \"${CACHE_NAME}\"|" "${FILTER_RS}"
echo "Stamped ${SW_FILE} and ${FILTER_RS}"

# ── 3. Build ──────────────────────────────────────────────────────────────────
echo "Running: dx build --release"
dx build --release

# ── 4. Copy static assets into build output ───────────────────────────────────
OUT="target/dx/profil/release/web/public"
echo "Copying static assets → ${OUT}/"
cp photo.png icon-192.png icon-512.png icon.svg favicon.ico manifest.json sw.js "${OUT}/"

echo ""
echo "✓ Build complete — CACHE_NAME: ${CACHE_NAME}"
echo "  Output: ${OUT}/"

# ── 5. Bundle for offline PWA ─────────────────────────────────────────────────
echo ""
echo "Bundling for offline PWA deployment…"
node bundle_sw.js -j -dioxus -logging "${OUT}" deploy profil
