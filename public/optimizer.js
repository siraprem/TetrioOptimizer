(function() {
    console.log('%c TETR.IO Optimizer: Performance Real Ativa ', 'background: #222; color: #bada55');
    
    // Apenas otimização comprovada: qualidade de imagem no scaling
    function applyImageOptimization() {
        const style = document.createElement('style');
        style.textContent = `
            /* Garantir nitidez visual no scaling do WebView */
            canvas, #game {
                image-rendering: pixelated;
                image-rendering: -webkit-optimize-contrast;
            }
        `;
        document.head.appendChild(style);
        
        console.log('[TETR.IO Optimizer] ✅ Otimização de Imagem Aplicada');
    }
    
    // Aplicar otimização após carregamento
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', applyImageOptimization);
    } else {
        applyImageOptimization();
    }
    
    // Monitoramento real de FPS (apenas logging)
    let frameCount = 0;
    let lastLog = performance.now();
    
    function monitorPerformance() {
        frameCount++;
        const now = performance.now();
        
        if (now - lastLog > 1000) { // Log a cada 1 segundo
            const fps = Math.round((frameCount * 1000) / (now - lastLog));
            console.log(`[TETR.IO Optimizer] 📊 FPS: ${fps}`);
            frameCount = 0;
            lastLog = now;
        }
        
        requestAnimationFrame(monitorPerformance);
    }
    
    // Iniciar monitoramento após 3 segundos
    setTimeout(() => {
        requestAnimationFrame(monitorPerformance);
    }, 3000);
})();