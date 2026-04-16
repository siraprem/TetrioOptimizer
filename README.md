# TETR.IO Optimizer

Um wrapper desktop para TETR.IO que carrega o jogo em uma webview e injeta scripts de otimização de performance. Construído com Tauri para Windows e Linux.

## O que faz

- Abre o TETR.IO em uma janela dedicada
- Injeta scripts para otimizar performance
- Corrige limite de FPS para 60Hz
- Funciona offline após o carregamento inicial

## Compilando a partir do código fonte

### Pré-requisitos

- Rust (última versão estável)
- Node.js 20+
- Tauri CLI

### Configuração

```bash
# Clone o repositório
git clone https://github.com/siraprem/TetrioOptimizer
cd TetrioOptimizer

# Instale as dependências
npm install

# Execute em modo de desenvolvimento
npm run tauri dev

# Compile para sua plataforma
npm run tauri build
```

## Builds automatizados

Este projeto usa GitHub Actions para builds automatizados. Quando você faz push de uma tag começando com `v` (como `v1.0.0`), o workflow irá:

1. Compilar para Windows (cria instaladores .msi e .exe)
2. Compilar para Linux (cria pacotes .deb e .AppImage)
3. Criar um Release no GitHub com todos os artifacts

Nota: O build ta quebrado nem tentem usar, é mais um placeholder pra no futuro arrumar algum dia (hoje nem perto)

## Requisito WebView2 no Windows

A versão Windows requer WebView2 Runtime. O instalador inclui um bootstrapper que irá instalá-lo automaticamente se necessário. Se você encontrar problemas com WebView2, consulte o guia de troubleshooting em `docs/WINDOWS_TROUBLESHOOTING.md`. (o troubleshooting ainda ta se fazendo calma la kkk)

## Formatos de distribuição

- **Windows**: Aplicativo .exe
- **Linux**: Executavel

## Licença

MIT License
