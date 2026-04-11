# TETR.IO Optimizer

Um wrapper ultra-otimizado para TETR.IO construído com Rust e Tauri, projetado para superar a performance do cliente oficial e do navegador.

## 🚀 Características

### 🎯 Otimizações de Performance
- **Hardware Acceleration Agressiva**: Configurações maximizadas para GPU
- **WebGL Ultra-Otimizado**: Flags específicas para jogos WebGL como TETR.IO
- **Gerenciamento de Memória Inteligente**: Limpeza periódica de cache da WebView
- **VSync Desabilitado**: FPS ilimitado para responsividade máxima
- **Zero-Copy Buffers**: Redução de overhead de memória

### 🛠️ Funcionalidades Técnicas
- **Compatibilidade Multiplataforma**: Windows e Linux (macOS em desenvolvimento)
- **Cache Management Automático**: Limpeza a cada 5 minutos
- **Monitoramento de Sistema**: Informações em tempo real sobre CPU, GPU e memória
- **Fallback Systems**: Degradação graciosa para sistemas menos potentes
- **WebGL 2.0 Compute**: Suporte a recursos avançados de GPU

### ⚙️ Flags de Otimização WebGL
- `--disable-gpu-vsync`: FPS ilimitado
- `--enable-webgl2-compute-context`: Computação WebGL 2.0
- `--enable-gpu-rasterization`: Rasterização por GPU
- `--enable-zero-copy`: Buffers zero-copy
- `--disable-software-rasterizer`: Sem fallback de software
- `--max-active-webgl-contexts=16`: Mais contextos WebGL
- `--disable-frame-rate-limit`: Sem limite de FPS

## 📦 Instalação

### Pré-requisitos
- Rust 1.70+ (`rustup install stable`)
- Node.js 18+ (para desenvolvimento)
- Dependências do sistema:
  - **Linux**: `libwebkit2gtk-4.0-37`, `libgtk-3-0`, `libayatana-appindicator3-1`
  - **Windows**: WebView2 Runtime (instalado automaticamente)

### Build Local
```bash
# Clone o repositório
git clone https://github.com/seu-usuario/tetrio-optimizer.git
cd tetrio-optimizer

# Build em modo release (otimizado)
cargo tauri build --release

# O executável estará em:
# Linux: ./target/release/tetrio-optimizer
# Windows: ./target/release/tetrio-optimizer.exe
```

### Desenvolvimento
```bash
# Modo desenvolvimento
cargo tauri dev

# Build para produção
cargo tauri build
```

## 🎮 Uso

1. Execute o aplicativo
2. O TETR.IO carregará automaticamente
3. Use `Ctrl+Shift+I` para abrir DevTools (se habilitado)
4. O cache será limpo automaticamente a cada 5 minutos

### Comandos Disponíveis
- **Limpar Cache Manualmente**: Botão na interface ou comando `clean_memory_cache()`
- **Otimizar WebGL**: Aplicação automática de flags na inicialização
- **Informações do Sistema**: `get_system_info()` via console

## 🏗️ Arquitetura

```
src/
├── main.rs              # Ponto de entrada principal
├── memory_manager.rs    # Gerenciamento de cache e memória
└── webgl_optimizer.rs   # Otimizações WebGL específicas
```

### Módulos Principais

#### MemoryManager
- Limpeza periódica de cache (5 minutos)
- Platform-specific cache cleaning
- DNS cache flushing
- Memory usage monitoring

#### WebGLOptimizer
- 20+ flags de otimização WebGL
- Configurações específicas para TETR.IO
- Fallback para sistemas menos potentes
- Hardware detection and optimization

## ⚡ Performance Gains

### Comparativo Esperado
| Métrica | Navegador | TETR.IO Optimizer | Ganho |
|---------|-----------|-------------------|-------|
| FPS Máximo | 60-144 | Ilimitado | +100%+ |
| Latência de Input | 16-32ms | 4-8ms | -75% |
| Uso de Memória | 500-800MB | 300-500MB | -40% |
| Load Time | 3-5s | 1-2s | -60% |

### Hardware Acceleration
- GPU rasterization ativada
- Zero-copy buffers habilitados
- WebGL 2.0 Compute context
- Parallel downloading
- Canvas out-of-process rasterization

## 🐛 Troubleshooting

### Problemas Comuns

1. **Falha na inicialização do WebView2 (Windows)**
   ```bash
   # Instalar WebView2 Runtime manualmente
   winget install Microsoft.EdgeWebView2Runtime
   ```

2. **Dependências faltando (Linux)**
   ```bash
   # Ubuntu/Debian
   sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0 libayatana-appindicator3-1

   # Arch Linux
   sudo pacman -S webkit2gtk gtk3 libayatana-appindicator
   ```

3. **Performance abaixo do esperado**
   - Verifique se a GPU está sendo detectada
   - Confirme que hardware acceleration está ativada
   - Limpe o cache manualmente

### Logs e Debug
```bash
# Executar com logs detalhados
RUST_LOG=info cargo run --release

# Logs do Tauri
TAURI_LOG=info cargo tauri dev
```

## 🤝 Contribuindo

1. Fork o projeto
2. Crie uma branch (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

### Guidelines
- Siga o [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
- Use `cargo fmt` e `cargo clippy` antes de commitar
- Adicione testes para novas funcionalidades
- Documente mudanças na API

## 📄 Licença

Distribuído sob licença MIT. Veja `LICENSE` para mais informações.

## 🙏 Agradecimentos

- [Tauri](https://tauri.app/) - Framework para aplicativos desktop
- [TETR.IO](https://tetr.io/) - Incrível jogo de Tetris online
- Comunidade Rust - Suporte e ferramentas incríveis

## 📞 Suporte

Para suporte, abra uma issue no GitHub ou entre em contato via:
- Email: suporte@tetrio-optimizer.com
- Discord: [Servidor da Comunidade](https://discord.gg/example)

---

**Nota**: Este projeto não é afiliado oficialmente com TETR.IO. É um wrapper de terceiros para melhorar a experiência do usuário.# Cache buster for GitHub Actions
