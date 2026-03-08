// Cache version — updated automatically by build.sh on every deploy.
// Format: profil-v0.YYYYMMDD.HHmm- (date+time ensures it always increases)
const CACHE_NAME = 'profil-v0.1000-';

self.addEventListener('install', event => {
    // Activate immediately, don't wait for the old SW to stop
    self.skipWaiting();
});

self.addEventListener('activate', event => {
    event.waitUntil(
        caches.keys().then(keys => {
            const oldCaches = keys.filter(k => k !== CACHE_NAME);
            const isUpdate = oldCaches.length > 0;

            // 1. Claim control first
            return self.clients.claim().then(() => {
                // 2. Only delete old caches AFTER claiming control
                return Promise.all(oldCaches.map(k => caches.delete(k)));
            }).then(() => {
                // 3. Notify the app to reload
                if (isUpdate) {
                    return self.clients.matchAll({ type: 'window' }).then(clients => {
                        clients.forEach(c => c.postMessage({ type: '__IFINTA_SW_UPDATED' }));
                    });
                }
            });
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
        !url.pathname.endsWith('/_bridge.js');

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
