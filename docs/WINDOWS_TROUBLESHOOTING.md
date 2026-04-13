# Solução de Problemas - Windows

## Requisitos do Sistema

O TETR.IO Optimizer requer o **Microsoft Edge WebView2 Runtime** para funcionar no Windows. Este é um componente oficial da Microsoft necessário para aplicativos modernos baseados em webview.

## Instalação Automática

O instalador do TETR.IO Optimizer (`.msi` ou `.exe`) tenta automaticamente:

1. **Verificar** se o WebView2 já está instalado
2. **Baixar** e instalar automaticamente se necessário
3. **Configurar** o aplicativo com todas as dependências

## Solução Manual (Se Necessário)

Se o instalador automático não funcionar ou se você estiver executando uma versão modificada do Windows, siga estes passos:

### Opção 1: Instalar WebView2 Manualmente

1. **Baixe o WebView2 Runtime** do site oficial da Microsoft:
   - [https://developer.microsoft.com/microsoft-edge/webview2/](https://developer.microsoft.com/microsoft-edge/webview2/)

2. **Execute o instalador**:
   - `MicrosoftEdgeWebView2RuntimeInstaller.exe` (para a versão permanente)
   - `MicrosoftEdgeWebView2RuntimeInstaller.exe /install` (para instalação silenciosa)

3. **Reinicie o computador** (recomendado)

### Opção 2: Usar o Bootstrapper do WebView2

Para sistemas corporativos ou com restrições:

1. **Baixe o bootstrapper**:
   - Link: [https://go.microsoft.com/fwlink/p/?LinkId=2124703](https://go.microsoft.com/fwlink/p/?LinkId=2124703)

2. **Execute como administrador**:
   ```cmd
   MicrosoftEdgeWebView2Setup.exe /install
   ```

### Opção 3: Verificar Instalação Existente

Para verificar se o WebView2 já está instalado:

1. **Pressione `Win + R`**
2. **Digite `appwiz.cpl`** e pressione Enter
3. **Procure por** "Microsoft Edge WebView2 Runtime" na lista de programas

## Erros Comuns e Soluções

### Erro: "WebView2Loader.dll não encontrada"

**Solução**:
1. Instale o WebView2 Runtime usando uma das opções acima
2. Se persistir, verifique se há bloqueadores de firewall/antivírus
3. Execute o Windows Update para obter atualizações do sistema

### Erro: "Falha ao inicializar WebView2"

**Solução**:
1. Execute o **Windows Update** para garantir todas as atualizações estão instaladas
2. Verifique se há espaço suficiente em disco (mínimo 100MB)
3. Execute o **SFC Scan**:
   ```cmd
   sfc /scannow
   ```

### Windows Modificado/OTIMIZADO

Para versões do Windows que removeram componentes essenciais:

1. **Recomendado**: Use uma versão padrão do Windows
2. **Alternativa**: Instale manualmente todos os componentes do .NET Framework e C++ Redistributables
3. **Links úteis**:
   - [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)
   - [.NET Framework 4.8](https://dotnet.microsoft.com/download/dotnet-framework/net48)

## Suporte Técnico

Se os problemas persistirem:

1. **Verifique os logs**:
   - O aplicativo cria logs em `%APPDATA%\tetrio-optimizer\logs\`

2. **Informações do sistema**:
   - Windows Version: `winver`
   - Arquitetura: 64-bit ou 32-bit

3. **Contate o suporte**:
   - Abra uma issue no GitHub: [https://github.com/seu-usuario/tetrio-optimizer/issues](https://github.com/seu-usuario/tetrio-optimizer/issues)
   - Inclua screenshots e informações do sistema

## Links Oficiais

- **WebView2 Runtime**: [https://developer.microsoft.com/microsoft-edge/webview2/](https://developer.microsoft.com/microsoft-edge/webview2/)
- **Documentação Tauri**: [https://tauri.app/](https://tauri.app/)
- **Repositório do Projeto**: [https://github.com/seu-usuario/tetrio-optimizer](https://github.com/seu-usuario/tetrio-optimizer)

## Notas Importantes

- O WebView2 é um componente **oficial da Microsoft** e **não contém malware**
- É necessário para aplicativos modernos como Discord, Teams, e muitos outros
- A instalação é **segura** e **recomendada pela Microsoft**
- O TETR.IO Optimizer **não funciona** sem o WebView2 no Windows