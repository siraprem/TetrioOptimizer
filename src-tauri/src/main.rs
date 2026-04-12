use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

fn main() {
    println!("[TETR.IO Optimizer] Starting...");

    tauri::Builder::default()
        .setup(|app| {
            println!("[TETR.IO Optimizer] App setup started");

            // Criar janela principal
            let window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://tetr.io".parse().unwrap()),
            )
            .title("TETR.IO Ultra Optimized")
            .inner_size(1280.0, 720.0)
            .center()
            .visible(true)
            .resizable(true)
            .fullscreen(false)
            .initialization_script(
                r#"
                console.log('[TETR.IO Optimizer] Script loaded - waiting for DOM...');
                
                // ===== ADBLOCK ULTRA =====
                const adDomains = [
                    'googleads', 'doubleclick', 'googlesyndication',
                    'analytics', 'telemetry', 'tracking', 'beacon',
                    'adservice', 'adsystem', 'adserver', 'adnxs',
                    'facebook.com/tr/', 'scorecardresearch', 'quantserve',
                    'amazon-adsystem', 'yieldmo', 'rubiconproject',
                    'pub.network', 'confiant', 'optimise', 'floors.dev',
                    'btloader', 'freestar', 'ceriad', 'darkside',
                    'openx.net', 'casalemedia', 'criteo', '3lift.com',
                    'imasdk.googleapis.com', 'yellowblue.io', 'pubmatic.com'
                ];
                
                // Interceptar fetch
                const originalFetch = window.fetch;
                window.fetch = function(...args) {
                    const url = args[0]?.url || args[0] || '';
                    if (typeof url === 'string') {
                        const lowerUrl = url.toLowerCase();
                        if (adDomains.some(domain => lowerUrl.includes(domain))) {
                            console.log('[Adblock] Blocked fetch:', url.substring(0, 80));
                            return Promise.reject(new Error('Blocked by TETR.IO Optimizer'));
                        }
                    }
                    return originalFetch.apply(this, args);
                };
                
                // Interceptar XMLHttpRequest
                const originalXHROpen = XMLHttpRequest.prototype.open;
                XMLHttpRequest.prototype.open = function(method, url) {
                    if (typeof url === 'string') {
                        const lowerUrl = url.toLowerCase();
                        if (adDomains.some(domain => lowerUrl.includes(domain))) {
                            console.log('[Adblock] Blocked XHR:', url.substring(0, 80));
                            this._blocked = true;
                            return;
                        }
                    }
                    return originalXHROpen.apply(this, arguments);
                };
                
                const originalXHRSend = XMLHttpRequest.prototype.send;
                XMLHttpRequest.prototype.send = function(body) {
                    if (this._blocked) {
                        console.log('[Adblock] Prevented blocked XHR from sending');
                        return;
                    }
                    return originalXHRSend.apply(this, arguments);
                };
                
                console.log('[TETR.IO Optimizer] Adblock active');
                
                // ===== REMOÇÃO AGGRESSIVA DE ELEMENTOS DE ANÚNCIO =====
                function removeAdElements() {
                    // Remover sticky footer (anúncio embaixo)
                    const stickyFooter = document.querySelector('[id*="sticky"], [class*="sticky"], [id*="footer"], [class*="footer"]');
                    if (stickyFooter) {
                        console.log('[Adblock] Removing sticky footer element');
                        stickyFooter.remove();
                    }
                    
                    // Remover elementos com classes/id de anúncio
                    const adSelectors = [
                        '[id*="ad-"]', '[class*="ad-"]', '[id*="ads"]', '[class*="ads"]',
                        '[id*="banner"]', '[class*="banner"]', '[id*="sponsor"]', '[class*="sponsor"]',
                        '[id*="promo"]', '[class*="promo"]', '[id*="commercial"]', '[class*="commercial"]',
                        'iframe[src*="ads"]', 'iframe[src*="doubleclick"]', 'iframe[src*="googleads"]',
                        '[id*="rail"]', '[class*="rail"]', // ingame-left_rail, ingame-right_rail
                        '[id*="ceriad"]', '[class*="ceriad"]' // Elementos do sistema de anúncios
                    ];
                    
                    adSelectors.forEach(selector => {
                        document.querySelectorAll(selector).forEach(el => {
                            console.log(`[Adblock] Removing ad element: ${selector}`);
                            el.remove();
                        });
                    });
                }
                
                // Executar remoção periodicamente
                setInterval(removeAdElements, 1000);
                
                // Monitorar mudanças no DOM para remover elementos novos
                const observer = new MutationObserver((mutations) => {
                    mutations.forEach((mutation) => {
                        if (mutation.addedNodes.length) {
                            removeAdElements();
                        }
                    });
                });
                
                // ===== FUNÇÕES PRINCIPAIS (executam quando DOM estiver pronto) =====
                function initOptimizer() {
                    console.log('[TETR.IO Optimizer] DOM ready, initializing...');
                    
                    // Observar o body quando estiver disponível
                    if (document.body) {
                        observer.observe(document.body, {
                            childList: true,
                            subtree: true
                        });
                    }
                    
                    // Teste - criar elemento vermelho (com retry se body não existir)
                    function addTestElement() {
                        if (!document.body) {
                            console.log('[TETR.IO Optimizer] Body not ready, retrying...');
                            setTimeout(addTestElement, 100);
                            return;
                        }
                        
                        const testDiv = document.createElement('div');
                        testDiv.textContent = 'TETR.IO Optimizer LOADED!';
                        testDiv.style.cssText = 'position:fixed;top:10px;left:10px;background:red;color:white;padding:10px;z-index:999999;font-size:16px;font-weight:bold;';
                        document.body.appendChild(testDiv);
                        
                        console.log('[TETR.IO Optimizer] ✅ Test element added!');
                        
                        // Remover após 10 segundos
                        setTimeout(() => {
                            if (testDiv.parentNode) {
                                testDiv.remove();
                                console.log('[TETR.IO Optimizer] Test element removed');
                            }
                        }, 10000);
                    }
                    
                    addTestElement();
                    
                    // ===== MENU VSYNC (F4) =====
                    let vsyncMenu = null;
                    let currentVSync = 120; // Valor padrão
                    
                    function createVSyncMenu() {
                        if (vsyncMenu) return;
                        
                        vsyncMenu = document.createElement('div');
                        vsyncMenu.style.cssText = `
                            position: fixed;
                            top: 50px;
                            right: 20px;
                            background: #1a1a1a;
                            color: white;
                            padding: 15px;
                            border-radius: 8px;
                            z-index: 999999;
                            font-family: Arial, sans-serif;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.5);
                            min-width: 200px;
                        `;
                        
                        vsyncMenu.innerHTML = `
                            <div style="margin-bottom: 10px; font-weight: bold; color: #4CAF50;">
                                🎮 VSync Control
                            </div>
                            <div style="margin-bottom: 10px; font-size: 12px; color: #aaa;">
                                Current: <span id="current-vsync">${currentVSync} Hz</span>
                            </div>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 5px;">
                                <button data-hz="60" style="padding: 8px; background: #333; border: none; color: white; border-radius: 4px; cursor: pointer;">60 Hz</button>
                                <button data-hz="75" style="padding: 8px; background: #333; border: none; color: white; border-radius: 4px; cursor: pointer;">75 Hz</button>
                                <button data-hz="120" style="padding: 8px; background: #4CAF50; border: none; color: white; border-radius: 4px; cursor: pointer;">120 Hz</button>
                                <button data-hz="144" style="padding: 8px; background: #333; border: none; color: white; border-radius: 4px; cursor: pointer;">144 Hz</button>
                                <button data-hz="165" style="padding: 8px; background: #333; border: none; color: white; border-radius: 4px; cursor: pointer;">165 Hz</button>
                                <button data-hz="240" style="padding: 8px; background: #333; border: none; color: white; border-radius: 4px; cursor: pointer;">240 Hz</button>
                            </div>
                            <div style="margin-top: 10px; font-size: 11px; color: #888; text-align: center;">
                                Press F4 to toggle
                            </div>
                        `;
                        
                        document.body.appendChild(vsyncMenu);
                        
                        // Adicionar event listeners aos botões
                        vsyncMenu.querySelectorAll('button').forEach(btn => {
                            btn.addEventListener('click', (e) => {
                                const hz = parseInt(e.target.dataset.hz);
                                currentVSync = hz;
                                document.getElementById('current-vsync').textContent = `${hz} Hz`;
                                
                                // Atualizar cores dos botões
                                vsyncMenu.querySelectorAll('button').forEach(b => {
                                    b.style.background = b.dataset.hz == hz ? '#4CAF50' : '#333';
                                });
                                
                                console.log(`[VSync] Set to ${hz} Hz`);
                                console.log(`[VSync] Applied ${hz} Hz (placeholder)`);
                            });
                        });
                    }
                    
                    function toggleVSyncMenu() {
                        if (!vsyncMenu) {
                            createVSyncMenu();
                        } else {
                            vsyncMenu.style.display = vsyncMenu.style.display === 'none' ? 'block' : 'none';
                        }
                        console.log('[VSync] Menu toggled');
                    }
                    
                    // Event listener para F4
                    document.addEventListener('keydown', (e) => {
                        if (e.key === 'F4' || e.key === 'f4') {
                            e.preventDefault();
                            e.stopPropagation();
                            toggleVSyncMenu();
                        }
                    }, true);
                    
                    console.log('[TETR.IO Optimizer] VSync menu ready (F4 to toggle)');
                }
                
                // Esperar DOM estar pronto
                if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', initOptimizer);
                } else {
                    initOptimizer();
                }
                "#
            )
            .build()?;

            println!("[TETR.IO Optimizer] ✅ Main window created!");
            println!("[TETR.IO Optimizer] TETR.IO should now be loading...");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}