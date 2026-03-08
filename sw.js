// Cache version — updated automatically by build.sh on every deploy.
// Format: profil-v0.YYYYMMDD.HHmm-COMMITHASH (date+time+git hash)
const CACHE_NAME = 'profil-v0.1000-';

// We don't use a pre-cache list because Dioxus generates hashed filenames
// (e.g. <project>-dxhABC123.js) that change with every build.
// Instead we cache at runtime: files are cached on first load.
self.addEventListener('message', event => {
    if (event.data && event.data.type === 'GET_VERSION') {
        event.ports[0].postMessage({ version: CACHE_NAME });
    }
});

self.addEventListener('install', event => {
    // Activate immediately, don't wait for the old SW to stop
    self.skipWaiting();
});

self.addEventListener('activate', event => {
    // 1. Claim clients IMMEDIATELY
    event.waitUntil(self.clients.claim());

    // 2. Then proceed with cleanup and notifications
    // Delete old cache versions
    event.waitUntil(
        caches.keys().then(keys => {
            const old = keys.filter(k => k !== CACHE_NAME);
            // Track whether this is a genuine update (old caches exist)
            const isUpdate = old.length > 0;
            return Promise.all(old.map(k => caches.delete(k))).then(() => isUpdate);
        }).then(isUpdate => {
            return self.clients.claim().then(() => isUpdate);
        }).then(isUpdate => {
            // Only notify clients to reload when we actually replaced an older version.
            // On iOS the SW can be terminated and re-activated by the OS —
            // that is NOT an update and must not trigger a reload loop.
            if (isUpdate) {
                return self.clients.matchAll({ type: 'window' }).then(clients => {
                    clients.forEach(c => c.postMessage({ type: '__IFINTA_SW_UPDATED', version: CACHE_NAME }));
                });
            }
        })
    );
});

self.addEventListener('fetch', event => {
    const url = new URL(event.request.url);

    // Navigation requests (HTML pages) → network-first
    // This way index.html always refreshes when there is a network connection
    if (event.request.mode === 'navigate') {
        event.respondWith(
            fetch(event.request)
                .then(response => {
                    if (response.status === 200) {
                        const clone = response.clone();
                        caches.open(CACHE_NAME).then(c => c.put(event.request, clone));
                    }
                    return response;
                })
                .catch(err => {
                    return caches.match(event.request)
                        .then(cached => {
                            if (cached) {
                                return cached;
                            }
                            return caches.match('index.html');
                        });
                })
        );
        return;
    }

    // Hashed assets (.js, .wasm) → cache-first
    // Their content never changes (guaranteed by the hash), so
    // it's enough to download once and always serve from cache afterwards.
    // IMPORTANT: exclude sw.js itself — the browser must always fetch it
    // fresh so it can detect updates and trigger the install event.
    const isCacheableAsset =
        /\.(js|wasm|css|png|jpg|svg|ico|woff2?)$/.test(url.pathname) &&
        !url.pathname.endsWith('/sw.js') &&
        !url.pathname.endsWith('/log_bridge.js');

    if (isCacheableAsset) {
        event.respondWith(
            caches.match(event.request).then(cached => {
                if (cached) return cached;

                // Not in cache, try network
                return fetch(event.request).then(response => {
                    if (response.status === 200) {
                        const clone = response.clone();
                        caches.open(CACHE_NAME).then(c => c.put(event.request, clone));
                    }
                    return response;
                }).catch(err => {
                    // If network fails, try ANY cache version before giving up
                    return caches.match(event.request);
                });
            })
        );
        return;
    }

    // Everything else (API calls, manifest.json, etc.) → network-only, fallback to cache
    event.respondWith(
        fetch(event.request)
            .then(response => {
                if (response.status === 200) {
                    const clone = response.clone();
                    caches.open(CACHE_NAME).then(c => c.put(event.request, clone));
                }
                return response;
            })
            .catch(err => {
                return caches.match(event.request);
            })
    );
});
