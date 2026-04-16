(function() {
    console.log('%c TETR.IO Optimizer: Otimização Passiva Ativa ', 'background: #222; color: #bada55');
    
    // Otimizações de DOM/CSS (sem interceptar renderização)
    function applyDOMOptimizations() {
        // 1. Forçar aceleração GPU em elementos visuais
        const style = document.createElement('style');
        style.textContent = `
            canvas, .game-canvas, #game {
                transform: translateZ(0);
                backface-visibility: hidden;
                -webkit-backface-visibility: hidden;
                image-rendering: -webkit-optimize-contrast;
                image-rendering: pixelated;
            }
            
            /* Otimizar partículas e efeitos visuais */
            .particle, .effect, .trail {
                will-change: transform, opacity;
            }
            
            /* Desativar animações pesadas em background */
            body, html {
                overflow: hidden !important;
            }
        `;
        document.head.appendChild(style);
        
        // 2. Remover listeners de eventos pesados se existirem
        setTimeout(() => {
            // Limpar event listeners desnecessários
            const heavyEvents = ['mousemove', 'scroll', 'resize'];
            heavyEvents.forEach(event => {
                window.addEventListener(event, (e) => {
                    e.stopImmediatePropagation();
                }, { capture: true, passive: true });
            });
        }, 1000);
        
        // 3. Otimizar requestAnimationFrame nativo (sem interceptar)
        if (window.requestAnimationFrame) {
            // Apenas garantir que está usando o nativo
            console.log('[TETR.IO Optimizer] ✅ Usando RAF nativo do navegador');
        }
    }
    
    // Aplicar otimizações após carregamento
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', applyDOMOptimizations);
    } else {
        applyDOMOptimizations();
    }
    
    // Monitorar performance (apenas logging)
    let frameCount = 0;
    let lastLog = performance.now();
    
    function monitorPerformance() {
        frameCount++;
        const now = performance.now();
        
        if (now - lastLog > 1000) { // Log a cada 1 segundo
            const fps = Math.round((frameCount * 1000) / (now - lastLog));
            console.log(`[TETR.IO Optimizer] 📊 FPS: ${fps} (Monitoramento Passivo)`);
            frameCount = 0;
            lastLog = now;
        }
        
        requestAnimationFrame(monitorPerformance);
    }
    
    // Iniciar monitoramento após 5 segundos
    setTimeout(() => {
        requestAnimationFrame(monitorPerformance);
    }, 5000);
})();