// Service Worker mínimo — necessário para o Chrome/Edge mostrar o botão "Instalar"
const CACHE = 'biblioteca-v1';

self.addEventListener('install', e => {
    e.waitUntil(
        caches.open(CACHE).then(c => c.addAll(['./index.html', './manifest.json']))
    );
    // Sem skipWaiting: evita forçar reload da página quando nova versão do SW chega
});

self.addEventListener('activate', e => {
    // Sem clients.claim(): evita interromper a sessão em andamento
    e.waitUntil(Promise.resolve());
});

self.addEventListener('fetch', e => {
    e.respondWith(
        fetch(e.request).catch(() => caches.match(e.request))
    );
});
