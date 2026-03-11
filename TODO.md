# todo:

## simple steps:
- A robot által generált tartalmak javítása, átfogalmazása. Túl robotos pár apróság, mint például a célkitűzés és a főbb erősségek

# for dev's:

## Architecture

The application targets **PWA (Progressive Web App) only** — all code compiles to WebAssembly and runs in the browser. The single `web` feature is the default. User selections are persisted in `localStorage`.

```
src/
├── main.rs                  # Entry point — Dioxus web launch
├── i18n.rs                  # Language enum (EN, HU, DE, FR, FI, ES, EL, IT)
└── ui/
    ├── mod.rs               # App component root — manages state & auto-save
    ├── view.rs              # Main layout: header, tabs, content, tab bar
    ├── state.rs             # Global state management & localStorage persistence
    ├── pdf.rs               # PDF generation — builds styled HTML & triggers print
    ├── tabs/
    │   ├── mod.rs           # Tab enum (Profile, Display, Filter)
    │   ├── profile.rs       # Language selector, role selector (Software Engineer / Project Owner / Test Manager)
    │   ├── display.rs       # Shows selected items: skills, companies, roles, tools, projects, characteristics
    │   └── filter.rs        # Checkbox filters for all categories; QR code; CACHE_NAME display
    └── i18n/
        ├── mod.rs           # UiI18n trait & factory function
        ├── english.rs       # EN translations
        ├── hungarian.rs     # HU translations (default)
        ├── german.rs        # DE translations
        ├── french.rs        # FR translations
        ├── finnish.rs       # FI translations
        ├── spanish.rs       # ES translations
        ├── greek.rs         # EL translations
        └── italian.rs       # IT translations

(root — static files and build tooling)
├── sw.js                    # Base Service Worker — offline caching, update detection
├── manifest.json            # PWA manifest (name: "Istvan Finta - Profil", theme: #17a2b8)
├── index.html               # PWA meta tags, SW registration, update banner
├── favicon.ico              # Browser tab icon (32×32)
├── icon-192.png             # PWA icon 192×192 (maskable)
├── icon-512.png             # PWA icon 512×512 (maskable)
├── icon.svg                 # Scalable vector icon
├── photo.png                # Profile photo (290×290)
├── bundle_sw.js             # Node.js bundling script — creates offline PWA deployment
└── build.sh                 # Build pipeline: stamp CACHE_NAME → dx build → copy statics → bundle
```

### Dependencies

| Crate | Purpose |
|-------|---------|
| `dioxus` 0.7 | Web UI framework (WASM) |
| `serde` + `serde_json` | State serialization (localStorage) |
| `web-sys` | Browser APIs (Storage, Window) |
| `qrcode` | SVG QR code generation |

### Service Worker Update Strategy

The SW (`sw.js`) handles offline caching and version management:

- **`index.html`** registers the SW with `updateViaCache: 'none'`
- A polling loop calls `reg.update()` every 10 seconds
- When a new SW activates, it posts `__PROFIL_SW_UPDATED` to clients
- The update banner shows "A new version is available" with a Refresh button (no auto-reload)
- **`CACHE_NAME`** is stamped by `build.sh` (e.g. `profil-20250308.1430-a1b2c3d4`)
- The SW uses runtime caching: network-first for navigation, cache-first for hashed assets
- iOS re-activation is distinguished from genuine updates to avoid reload loops

### Bundled Offline Deployment (bundle_sw.js)

For GitHub Pages and similar static hosting, `bundle_sw.js` creates a fully
self-contained deployment from the build output. The result is just two
physical files: `index.html` and `sw.js`.

**How it works (JSON-in-HTML mode, `-j` flag):**

1. All files are gzip-compressed and base64-encoded
2. A bootloader `index.html` is generated that registers the SW and shows a
   loading spinner
3. The actual app `index.html` is itself gzip+base64 encoded and embedded in
   the bootloader — it contains all the asset entries
4. On SW `install`, assets are decoded from the embedded JSON map and stored
   in CacheStorage
5. On `activate`, the SW intercepts all fetch requests and serves from cache
6. PWA metadata (manifest, icons) is embedded as data URIs in the bootloader
   so PWA install works even before the SW activates

**Modifier flags:**

| Flag | Purpose |
|------|---------|
| `-j` | JSON-in-HTML bundling (assets embedded in SW) |
| `-dioxus` | SPA routing — all navigation → `index.html` (no multi-page resolution) |
| `-logging` | Injects log ring buffer + forwarding into SW |
| `-raw` | Copy manifest/icons as physical files instead of embedding as data URIs |

**Build pipeline (`build.sh`):**

```bash
# 1. Stamp CACHE_NAME with date+time+commit into sw.js and filter.rs
# 2. dx build --release
# 3. Copy root static files (sw.js, photo.png, icons, manifest.json) into build output
# 4. Run: node bundle_sw.js -j -dioxus -logging <build-output> deploy profil
# 5. Output: deploy/profil/{index.html, sw.js}
```

**GitHub Pages deployment:**

The `.github/workflows/deploy.yml` workflow runs `build.sh` on push to `main`
and deploys `deploy/profil/` as a GitHub Pages artifact.

To enable: repo Settings → Pages → Source → GitHub Actions.

Live: https://ifinta.github.io/profil/

### Internationalization (i18n)

Every user-facing string is abstracted behind the `UiI18n` trait, with a factory function selecting the correct implementation based on the active `Language`.

**8 supported languages:** English, Hungarian (default), German, French, Finnish, Spanish, Greek, Italian

**Adding a new language** requires two steps:

1. Add a variant to the `Language` enum in `src/i18n.rs`
2. Create a new implementation file in `src/ui/i18n/` and register it in the factory function

## Target Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| Web (WASM/PWA) | ✅ Supported | Primary target — installable via browser |
| iOS Safari (PWA) | ✅ Supported | Share → "Add to Home Screen" |
| Android Chrome (PWA) | ✅ Supported | Menu → "Add to Home screen" |
| Desktop Chrome/Edge (PWA) | ✅ Supported | Address bar install icon |

## Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# Dioxus CLI
cargo install dioxus-cli

# Node.js (>= 18, for bundle_sw.js)
```

## Build & Run

```bash
# Clone the repository
git clone https://github.com/ifinta/profil.git
cd profil

# Development server with hot-reload
dx serve --platform web

# Full release build + bundled offline deployment
./build.sh
# Output:
#   deploy/profil/ — bundled offline deployment (index.html + sw.js only)

# Serve locally:
npx serve deploy/ -l 8080
# → http://localhost:8080/profil/
```

### Feature Flag

| Flag | Description |
|------|-------------|
| `web` | Browser PWA via WASM (default) — Dioxus web, web-sys localStorage, QR code generation |
