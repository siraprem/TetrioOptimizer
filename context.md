# TETR.IO Optimizer - Context Checkpoint

## 📍 Estado Atual
**Projeto estável e funcional** - App abre TETR.IO sem travamentos no Wayland (KDE Plasma 6)

### ✅ O que funciona:
1. **Janela principal** - Abre imediatamente com TETR.IO carregando
2. **VSync fixo 60Hz** - Usa configuração padrão do sistema (sem controles complexos)
3. **Elemento de teste** - Div vermelho confirma execução do script
4. **Sem DevTools automático** - Config correto no tauri.conf.json
5. **Build cross-platform** - Compila para Windows (x86_64-pc-windows-gnu)

### 📁 Estrutura de Pastas:
```
TetrioOptimizer/
├── src-tauri/
│   ├── src/main.rs          # Código principal (estável)
│   ├── Cargo.toml           # Dependências Tauri 2.0
│   ├── tauri.conf.json      # Configuração do app
│   ├── build.rs             # Script de build
│   └── icons/               # Ícones do app
├── target/                  # Binários compilados
│   ├── debug/tetrio-optimizer      # Linux
│   └── x86_64-pc-windows-gnu/release/tetrio-optimizer.exe  # Windows
├── Cargo.toml               # Workspace config
└── context.md               # Este arquivo
```

## 💻 Hardware & Ambiente
- **Sistema:** Arch Linux
- **Desktop:** KDE Plasma 6 (Wayland)
- **GPU:** AMD RX 550
- **Monitor:** 60Hz (limitação física para testes VSync)

## 🛠️ Stack Técnica
- **Framework:** Tauri 2.0 (Rust)
- **Frontend:** WebView apontando para https://tetr.io
- **Injeção:** JavaScript via `window.eval()` no initialization_script
- **Build:** Cargo + Rust toolchain

## 📜 Código-Fonte Estável

### `src-tauri/src/main.rs` (versão simplificada):
```rust
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

            // Script de inicialização simples
            let init_script = r#"
                console.log('[TETR.IO Optimizer] Script loaded - waiting for DOM...');
                
                function initOptimizer() {
                    console.log('[TETR.IO Optimizer] DOM ready, initializing...');
                    console.log('[TETR.IO Optimizer] VSync fixed at 60Hz (system default)');
                    console.log('[TETR.IO Optimizer] Ready!');
                }
                
                // Elemento de teste
                function addTestElement() {
                    const testEl = document.createElement('div');
                    testEl.id = 'tetrio-optimizer-test';
                    testEl.style.cssText = `
                        position: fixed;
                        top: 10px;
                        left: 10px;
                        background: #c33;
                        color: white;
                        padding: 5px 10px;
                        border-radius: 4px;
                        font-family: Arial, sans-serif;
                        font-size: 12px;
                        z-index: 999998;
                        opacity: 0.9;
                    `;
                    testEl.textContent = 'TETR.IO Optimizer LOADED!';
                    document.body.appendChild(testEl);
                    console.log('[TETR.IO Optimizer] ✅ Test element added!');
                    
                    setTimeout(() => {
                        if (testEl.parentNode) {
                            testEl.parentNode.removeChild(testEl);
                            console.log('[TETR.IO Optimizer] Test element removed');
                        }
                    }, 5000);
                }
                
                // Inicialização
                if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', () => {
                        initOptimizer();
                        addTestElement();
                    });
                } else {
                    initOptimizer();
                    addTestElement();
                }
            "#;

            window.eval(&*init_script).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### `src-tauri/Cargo.toml`:
```toml
[package]
name = "tetrio-optimizer"
version = "0.2.0"
description = "TETR.IO Optimizer - Performance enhancements for TETR.IO"
authors = ["You"]
license = "MIT"
repository = ""
edition = "2021"

[build-dependencies]
tauri-build = { version = "2.5.6" }

[dependencies]
tauri = { version = "2.10.3", features = ["devtools"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
sysinfo = "0.30"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

## ⚠️ O que NÃO fazer (lições aprendidas):
1. **NÃO** tentar adblock via Rust - Quebra configs do jogo (pub.network)
2. **NÃO** abrir DevTools automaticamente - Interfere com input do jogo
3. **NÃO** menus VSync complexos - Beta testers reportaram não funcionar
4. **NÃO** flags WebGL problemáticas - `--disable-frame-rate-limit` causa conflito no Wayland
5. **NÃO** múltiplas janelas - Causa travamento no Wayland

## 🚀 Próximo Passo (quando reiniciar):
1. **Testar build Windows** - Verificar se .exe funciona corretamente
2. **Adicionar adblock leve** - Apenas remoção DOM (sem bloquear requests)
3. **Otimizações WebGL seguras** - Apenas flags comprovadamente estáveis
4. **Menu de contexto nativo** - Para abrir DevTools manualmente se necessário
5. **Preparar para distribuição** - Criar .deb para Linux e .msi para Windows

## 🔧 Comandos Úteis:
```bash
# Build Linux
cargo build --release

# Build Windows (cross-compile)
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu

# Executar
./target/release/tetrio-optimizer

# Git
git add . && git commit -m "DESCRIÇÃO" && git push
```

## 📅 Última Modificação:
- **Data:** 12/04/2026
- **Status:** Estável e funcional
- **Versão:** VSync fixo 60Hz, sem adblock, sem menus complexos
- **Pronto para:** Testes beta e distribuição básica
```

**Checkpoint criado com sucesso!** O arquivo `context.md` agora contém todo o contexto necessário para continuar o desenvolvimento quando você reiniciar. 🚀