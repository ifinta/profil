#!/usr/bin/env bash
# build.sh — build profil and prepare output for deployment
#
# What it does:
#   1. Stamps sw.js and rust code because of the new version
#   2. Runs `dx build --release`
#   3. Copies static assets into the build output
#
# Usage:
#   ./build.sh          — full build
#   ./build.sh --dry    — print the new VERSION without building

set -euo pipefail

DRY=false
[[ "${1:-}" == "--dry" ]] && DRY=true

# ── 1. Generate CACHE_NAME ────────────────────────────────────────────────────
BUILD_TS="$(date +%Y%m%d.%H%M)"
GIT_HASH="$(git rev-parse --short=8 HEAD)"
APP_NAME="profil"
APP_VERSION="${APP_NAME}-${BUILD_TS}-${GIT_HASH}"
SW_FILE="sw.js"
FILTER_RS="src/ui/tabs/filter.rs"

echo "APP_VERSION → ${APP_VERSION}"
$DRY && exit 0

# ── 2. Stamp CACHE_NAME into sw.js ───────────────────────────────────────────
sed -i "s|.*var APP_VERSION =.*|var APP_VERSION = '${APP_VERSION}';|"  "${SW_FILE}"
sed -i "s|.*APP_VERSION.*|                \"${APP_VERSION}\" // APP_VERSION|" "${FILTER_RS}"
echo "Stamped ${SW_FILE} and ${FILTER_RS}"

# ── 3. Build ──────────────────────────────────────────────────────────────────
echo "Running: dx build --release"
dx build --release

# ── 4. Copy static assets into build output ───────────────────────────────────
OUT="target/dx/${APP_NAME}/release/web/public"
echo "Copying static assets → ${OUT}/"
cp photo.png icon-192.png icon-512.png icon.svg favicon.ico manifest.json sw.js "${OUT}/"

echo ""
echo "✓ Build complete — APP_VERSION: ${APP_VERSION}"
echo "  Output: ${OUT}/"

# ── 5. Bundle for offline PWA ─────────────────────────────────────────────────
echo ""
echo "Bundling for offline PWA deployment…"
node bundle.js "${OUT}" deploy "${APP_NAME}"
rm -rf "${OUT}/"

# ── 6. Copy icons, manifest file into deploy folder ───────────────────────────
echo ""
echo "  Copying icons, manifest → deploy/${APP_NAME}/"
cp icon-192.png icon-512.png icon.svg favicon.ico manifest.json "deploy/${APP_NAME}/"
