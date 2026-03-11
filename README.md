# István Finta - profil

Usage: First tab: base data. Please select in the second tab, what should be listed on the third tab

## Installation

### iOS

- In the Safari open the PWA App from web
- select Share button
- select "To Home Screen"
- The App Icon will be reachable on the Home Screen, the App get the latest version of the code automatically

### Android

- In Chrome open the PWA App from web
- Tap the three-dot menu (⋮) in the top-right corner
- Select "Add to Home screen" or "Install app"
- The App Icon will be reachable on the Home Screen, the App gets the latest version of the code automatically

## Build & Deployment

The app is built with Rust/Dioxus (WebAssembly) and deployed as an offline PWA.

`build.sh` compiles the app, then `bundle_sw.js` compresses everything into a
self-contained deployment with just `index.html` and `sw.js`. All assets are
embedded inside the service worker — the app works fully offline from the first
visit.

```bash
# Prerequisites: Rust, wasm32-unknown-unknown target, Dioxus CLI, Node.js >= 18

# Build + bundle:
./build.sh
# Output: deploy/profil/

# Serve locally:
npx serve deploy/ -l 8080
# → http://localhost:8080/profil/
```

The CI workflow (`.github/workflows/deploy.yml`) builds and deploys to GitHub
Pages on every push to `main`.

## Related Repositories

- [the PWA App](https://ifinta.github.io/profil) — A running version of this App
- [zsozso-dioxus](https://github.com/ifinta/zsozso-dioxus) — Zsozso Wallet (same PWA bundling pattern)
- [zsozso-webpage](https://github.com/ifinta/zsozso-webpage) — Iceberg Protocol website
