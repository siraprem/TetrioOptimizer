[English version available here](./README_EN.md)

# TETR.IO Optimizer - Performance Absurda no Desktop

Um wrapper desktop feito em Tauri que roda o TETR.IO com otimizações de performance que o navegador normal não tem. Basicamente é o TETR.IO, mas mais rápido, mais leve e sem aquelas travadas chatas.

## O que é isso?

Sabe o TETR.IO, aquele jogo de Tetris online que todo mundo joga no navegador? Pois é, esse projeto é uma tentativa de fazer um app desktop que roda o jogo direto, mas com um monte de truques por baixo dos panos pra deixar tudo mais fluido.

## Funcionalidades

- **Aceleração de GPU Nativa:** Configurações específicas pra drivers Mesa (AMD/Intel) no Linux. A placa de vídeo trabalha de verdade.
- **V-Sync Bypass:** Desativação do vblank no nível do kernel pra resposta instantânea. Sem delay entre o seu comando e a peça se mover.
- **Otimização Passiva de DOM:** Injeção de CSS que remove efeitos pesados do TETR.IO sem interferir na lógica do jogo (Safe pra sua conta!).
- **Tauri 2.0 Engine:** Ocupa menos RAM e CPU que o cliente oficial ou o navegador aberto. É magia, mas com código.
- **Build Cross-Platform:** Roda no Linux e gera executável pra Windows direto do Arch. Sem VM, sem dor de cabeça.

## Qual versão baixar?

A gente tem duas branches principais agora. O projeto evoluiu e a estrutura permite escolher entre estabilidade e performance bruta:

### Versão Main (Estabilidade) - **RECOMENDADA PRA MAIORIA**
Essa é a versão onde a mágica acontece de forma controlada. Ela usa todas as otimizações mas mantém o FPS limitado a 60Hz pra evitar problemas.

- **Foco:** Performance otimizada com estabilidade garantida.
- **Otimizações:** Aceleração GPU, bypass de V-Sync, otimizações de DOM.
- **Para quem:** Todo mundo que quer jogar sem travadas mas sem surpresas.

### Versão Performance-Uncapped
Essa versão é o nosso "modo turbo". Tira todos os limites de FPS e desativa tudo que puder pra performance máxima.

- **Foco:** FPS ilimitado pra monitores de alta taxa (144Hz+).
- **Otimizações:** Tudo da Main + `--disable-frame-rate-limit` + `vblank_mode=0` agressivo.
- **Para quem:** Quem tem monitor gamer e quer cada frame possível.

**Dica:** Se você não sabe o que é refresh rate ou não tem monitor acima de 60Hz, fica na Main que é sucesso.

## Como baixar

Pra baixar o aplicativo, acesse nossa página de lançamentos:

**[Clique aqui pra ver todas as versões (Releases)](https://github.com/siraprem/TetrioOptimizer/releases)**

**Dica de ouro:** Escolhe entre a normal (main) e uncapped. se tiver duvidas escolha normal.

## Como rodar e buildar

### Pré-requisitos
- Rust (última versão estável)
- Node.js 20+
- Tauri CLI (`npm install -g @tauri-apps/cli`)

### Build no Linux (pra Linux)
```bash
# Baixa o código
git clone https://github.com/siraprem/TetrioOptimizer.git
cd TetrioOptimizer

# Instala as dependências
npm install

# Build normal (Main branch)
npm run tauri build

# Ou se quiser a versão turbo:
git checkout performance-uncapped
npm run tauri build
```

### Build pra Windows (direto do Arch)
```bash
# Instala o target Windows
rustup target add x86_64-pc-windows-gnu

# Instala o linker Mingw (no Arch)
sudo pacman -S mingw-w64-gcc

# Build cross-compile
npx tauri build --target x86_64-pc-windows-gnu
```

O executável Windows vai aparecer em `target/x86_64-pc-windows-gnu/release/tetrio-optimizer.exe`.

## Desenvolvimento Assistido

O TETR.IO Optimizer utiliza um fluxo de desenvolvimento AI-Augmented (Desenvolvimento Aumentado por IA). Isso permite que eu, como desenvolvedora, foque na arquitetura e na experiência do usuário, enquanto utilizo modelos de linguagem avançados para otimizar a implementação, diagnosticar problemas de performance e acelerar a resolução de bugs específicos de plataforma.

## Estrutura do projeto

```
TetrioOptimizer/
├── src-tauri/          # Código Rust do Tauri (onde a mágica acontece)
├── public/             # Scripts de otimização que são injetados
├── .cargo/             # Configurações de cross-compile
├── package.json        # Dependências do Node
└── README.md           # Esse arquivo aqui
```

## Licença

**MIT** - basicamente pode fazer o que quiser com o código, só não me processa por favor, recomendo ler o LICENSE.md 

## Aviso legal

**Isso aqui é um projeto não-oficial.** O TETR.IO é dos criadores originais. Esse app não tem nenhuma afiliação com a equipe do TETR.IO, é só um wrapper feito por fã pra fã. Não me deem strike, eu amo o jogo.

Se puder, apoia o osk (dev do jogo) lá no site oficial! O cara mantém os servidores com anúncios e doações.