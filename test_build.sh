#!/bin/bash
echo "🚀 Simulando GitHub Actions Build..."
echo "======================================"

# Limpar cache local
echo "🧹 Limpando cache..."
cargo clean

# Verificar estrutura
echo "📁 Verificando estrutura..."
echo "Arquivos Cargo.toml encontrados:"
find . -name "Cargo.toml" -type f

echo ""
echo "📦 Verificando workspace..."
cargo metadata --format-version=1 2>&1 | head -20

echo ""
echo "🔧 Tentando build local..."
cargo build --release --target x86_64-pc-windows-msvc 2>&1 | tail -50

echo ""
echo "✅ Teste concluído!"
