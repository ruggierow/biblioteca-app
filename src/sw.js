// Service Worker mínimo — necessário para o Chrome/Edge mostrar o botão "Instalar"
// __VERSION__ é substituído pelo processo de build (sed/PowerShell) com a versão real
const CACHE = 'biblioteca-__VERSION__';

self.addEventListener('install', e => {
    e.waitUntil(
        caches.open(CACHE).then(c => c.addAll(['./index.html', './manifest.json']))
    );
    self.skipWaiting(); // Força ativação imediata, descartando o SW antigo
});

self.addEventListener('activate', e => {
    // Apaga caches de versões anteriores e assume controle da página
    e.waitUntil(
        caches.keys()
            .then(keys => Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k))))
            .then(() => self.clients.claim())
    );
});

self.addEventListener('fetch', e => {
    e.respondWith(
        fetch(e.request).catch(() => caches.match(e.request))
    );
});
