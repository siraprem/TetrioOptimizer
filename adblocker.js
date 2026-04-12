// Adblocker para TETR.IO
// Remove elementos de anúncios e bloqueia requisições para domínios de publicidade

(function() {
    'use strict';
    
    console.log('[TETR.IO Adblocker] Inicializando...');
    
    // Lista de domínios de publicidade para bloquear
    const adDomains = [
        'ads.tetr.io',
        'adservice.google.com',
        'doubleclick.net',
        'googlesyndication.com',
        'google-analytics.com',
        'facebook.com/tr',
        'analytics.tetr.io',
        'tracking.',
        'ad.',
        'ads.',
        'adserver.',
        'advertising.',
        'banner.',
        'promo.',
        'sponsor.',
        'track.',
        'pixel.',
        'beacon.'
    ];
    
    // Seletores CSS para elementos de anúncios no TETR.IO
    const adSelectors = [
        // Anúncios comuns no TETR.IO
        '.ad-container',
        '.ad-banner',
        '.ad-sidebar',
        '.advertisement',
        '[class*="ad-"]',
        '[class*="ads"]',
        '[id*="ad-"]',
        '[id*="ads"]',
        // Anúncios específicos do TETR.IO (baseado em análise)
        '.tetr-ads',
        '.tetrio-ad',
        '.game-ad',
        '.premium-promo',
        '.donation-banner',
        '.sponsor-notice'
    ];
    
    // Interceptar requisições fetch
    const originalFetch = window.fetch;
    window.fetch = function(...args) {
        const url = args[0] instanceof Request ? args[0].url : args[0];
        
        // Verificar se a URL contém domínios de anúncios
        if (typeof url === 'string' && adDomains.some(domain => url.includes(domain))) {
            console.log('[Adblocker] Bloqueando requisição para:', url);
            return Promise.reject(new Error('Bloqueado pelo Adblocker'));
        }
        
        return originalFetch.apply(this, args);
    };
    
    // Interceptar requisições XMLHttpRequest
    const originalXHROpen = XMLHttpRequest.prototype.open;
    XMLHttpRequest.prototype.open = function(method, url, ...rest) {
        if (typeof url === 'string' && adDomains.some(domain => url.includes(domain))) {
            console.log('[Adblocker] Bloqueando XHR para:', url);
            this._blocked = true;
            return;
        }
        return originalXHROpen.apply(this, [method, url, ...rest]);
    };
    
    const originalXHRSend = XMLHttpRequest.prototype.send;
    XMLHttpRequest.prototype.send = function(...args) {
        if (this._blocked) {
            console.log('[Adblocker] Ignorando XHR bloqueado');
            return;
        }
        return originalXHRSend.apply(this, args);
    };
    
    // Função para remover elementos de anúncios
    function removeAds() {
        let removedCount = 0;
        
        adSelectors.forEach(selector => {
            try {
                document.querySelectorAll(selector).forEach(element => {
                    // Verificar se é realmente um anúncio (tamanho, conteúdo, etc.)
                    const style = window.getComputedStyle(element);
                    const isAd = 
                        element.textContent.includes('ad') ||
                        element.textContent.includes('Ad') ||
                        element.textContent.includes('AD') ||
                        element.textContent.includes('sponsor') ||
                        element.textContent.includes('Sponsor') ||
                        element.textContent.includes('promo') ||
                        element.textContent.includes('Promo') ||
                        element.getAttribute('data-ad') ||
                        element.getAttribute('data-ad-client');
                    
                    if (isAd || element.offsetWidth > 100 || element.offsetHeight > 50) {
                        element.style.display = 'none';
                        element.remove();
                        removedCount++;
                    }
                });
            } catch (e) {
                // Ignorar erros de seletores inválidos
            }
        });
        
        if (removedCount > 0) {
            console.log(`[Adblocker] Removidos ${removedCount} elementos de anúncios`);
        }
    }
    
    // Observar mudanças no DOM para remover anúncios dinamicamente
    const observer = new MutationObserver((mutations) => {
        let shouldCheck = false;
        
        mutations.forEach(mutation => {
            if (mutation.addedNodes.length > 0) {
                shouldCheck = true;
            }
        });
        
        if (shouldCheck) {
            setTimeout(removeAds, 100);
        }
    });
    
    // Iniciar observação
    observer.observe(document.documentElement, {
        childList: true,
        subtree: true
    });
    
    // Remover anúncios inicialmente
    window.addEventListener('DOMContentLoaded', removeAds);
    window.addEventListener('load', removeAds);
    
    // Executar periodicamente para garantir que anúncios sejam removidos
    setInterval(removeAds, 5000);
    
    console.log('[TETR.IO Adblocker] Configurado com sucesso!');
})();