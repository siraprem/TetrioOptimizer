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
            .build()?;

            println!("[TETR.IO Optimizer] ✅ Main window created!");
            println!("[TETR.IO Optimizer] TETR.IO should now be loading...");

            // ===== SCRIPT DE INICIALIZAÇÃO SIMPLES =====
            // APENAS VSync menu - SEM ADBLOCK por enquanto
            let init_script = r#"
                console.log('[TETR.IO Optimizer] Script loaded - waiting for DOM...');
                
                // Função para inicializar quando DOM estiver pronto
                function initOptimizer() {
                    console.log('[TETR.IO Optimizer] DOM ready, initializing...');
                    
                    // ===== VSYNC MENU =====
                    let vsyncMenu = null;
                    let currentVSync = 120; // Valor padrão
                    
                    // Criar menu VSync
                    function createVSyncMenu() {
                        vsyncMenu = document.createElement('div');
                        vsyncMenu.id = 'tetrio-vsync-menu';
                        vsyncMenu.style.cssText = `
                            position: fixed;
                            top: 20px;
                            right: 20px;
                            background: rgba(0, 0, 0, 0.85);
                            color: white;
                            padding: 15px;
                            border-radius: 8px;
                            z-index: 999999;
                            font-family: Arial, sans-serif;
                            font-size: 14px;
                            box-shadow: 0 4px 12px rgba(0,0,0,0.3);
                            display: none;
                            min-width: 180px;
                        `;
                        
                        vsyncMenu.innerHTML = `
                            <div style="margin-bottom: 10px; font-weight: bold; border-bottom: 1px solid #444; padding-bottom: 5px;">
                                VSync Settings
                            </div>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 8px;">
                                <button data-hz="60" style="padding: 8px; background: #333; border: 1px solid #555; color: #ccc; border-radius: 4px; cursor: pointer;">60 Hz</button>
                                <button data-hz="75" style="padding: 8px; background: #333; border: 1px solid #555; color: #ccc; border-radius: 4px; cursor: pointer;">75 Hz</button>
                                <button data-hz="120" style="padding: 8px; background: #2a5; border: 1px solid #3c6; color: white; border-radius: 4px; cursor: pointer; font-weight: bold;">120 Hz</button>
                                <button data-hz="144" style="padding: 8px; background: #333; border: 1px solid #555; color: #ccc; border-radius: 4px; cursor: pointer;">144 Hz</button>
                                <button data-hz="165" style="padding: 8px; background: #333; border: 1px solid #555; color: #ccc; border-radius: 4px; cursor: pointer;">165 Hz</button>
                                <button data-hz="240" style="padding: 8px; background: #333; border: 1px solid #555; color: #ccc; border-radius: 4px; cursor: pointer;">240 Hz</button>
                            </div>
                            <div style="margin-top: 10px; font-size: 12px; color: #aaa; text-align: center;">
                                Press F4 to toggle
                            </div>
                        `;
                        
                        document.body.appendChild(vsyncMenu);
                        
                        // Adicionar event listeners aos botões
                        vsyncMenu.querySelectorAll('button').forEach(btn => {
                            btn.addEventListener('click', function() {
                                const hz = parseInt(this.getAttribute('data-hz'));
                                setVSync(hz);
                            });
                        });
                    }
                    
                    // Definir VSync
                    function setVSync(hz) {
                        console.log('[VSync] Set to', hz, 'Hz');
                        currentVSync = hz;
                        
                        // Atualizar UI
                        if (vsyncMenu) {
                            vsyncMenu.querySelectorAll('button').forEach(btn => {
                                const btnHz = parseInt(btn.getAttribute('data-hz'));
                                if (btnHz === hz) {
                                    btn.style.background = '#2a5';
                                    btn.style.border = '1px solid #3c6';
                                    btn.style.color = 'white';
                                    btn.style.fontWeight = 'bold';
                                } else {
                                    btn.style.background = '#333';
                                    btn.style.border = '1px solid #555';
                                    btn.style.color = '#ccc';
                                    btn.style.fontWeight = 'normal';
                                }
                            });
                        }
                        
                        // Aplicar VSync (placeholder - precisa do comando Rust)
                        console.log('[VSync] Applied', hz, 'Hz (placeholder)');
                    }
                    
                    // Toggle menu com F4
                    function toggleVSyncMenu() {
                        if (!vsyncMenu) createVSyncMenu();
                        
                        const isVisible = vsyncMenu.style.display === 'block';
                        vsyncMenu.style.display = isVisible ? 'none' : 'block';
                        console.log('[VSync] Menu toggled', { visible: !isVisible });
                    }
                    
                    // Event listener para F4
                    document.addEventListener('keydown', function(e) {
                        if (e.key === 'F4') {
                            e.preventDefault();
                            toggleVSyncMenu();
                        }
                    });
                    
                    // Criar menu inicialmente oculto
                    createVSyncMenu();
                    console.log('[TETR.IO Optimizer] VSync menu ready (F4 to toggle)');
                    
                    // ===== ELEMENTO DE TESTE =====
                    // Apenas para confirmar que o script está funcionando
                    const testElement = document.createElement('div');
                    testElement.id = 'tetrio-optimizer-test';
                    testElement.style.cssText = `
                        position: fixed;
                        top: 10px;
                        left: 10px;
                        background: #f00;
                        color: white;
                        padding: 5px 10px;
                        border-radius: 4px;
                        font-family: Arial, sans-serif;
                        font-size: 12px;
                        z-index: 999999;
                    `;
                    testElement.textContent = 'TETR.IO Optimizer LOADED!';
                    document.body.appendChild(testElement);
                    
                    // Remover após 5 segundos
                    setTimeout(() => {
                        if (testElement.parentNode) {
                            testElement.remove();
                            console.log('[TETR.IO Optimizer] Test element removed');
                        }
                    }, 5000);
                }
                
                // Aguardar DOM estar pronto
                if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', initOptimizer);
                } else {
                    initOptimizer();
                }
            "#;

            // Injetar script
            window.eval(&init_script).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}