#!/usr/bin/env python3
"""
Script para mover WebView2Loader.dll para o lado do executável após build.
Executar após: cargo tauri build
"""

import os
import shutil
import sys
import glob

def main():
    print("🔧 Movendo WebView2Loader.dll para o lado do executável...")
    
    # Caminhos
    dll_source = "src-tauri/resources/WebView2Loader.dll"
    
    # Encontrar executáveis gerados
    exe_patterns = [
        "src-tauri/target/**/tetrio-optimizer.exe",
        "src-tauri/target/**/bundle/**/*.exe",
        "src-tauri/target/**/bundle/**/*.msi"
    ]
    
    exe_found = []
    
    for pattern in exe_patterns:
        for exe_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(exe_path):
                exe_found.append(exe_path)
    
    if not exe_found:
        print("❌ Nenhum executável encontrado!")
        return 1
    
    # Mover DLL para cada executável encontrado
    for exe_path in exe_found:
        exe_dir = os.path.dirname(exe_path)
        dll_dest = os.path.join(exe_dir, "WebView2Loader.dll")
        
        try:
            # Copiar DLL
            shutil.copy2(dll_source, dll_dest)
            print(f"✅ DLL copiada para: {dll_dest}")
            
            # Verificar se copiou
            if os.path.exists(dll_dest):
                print(f"   📁 Tamanho: {os.path.getsize(dll_dest)} bytes")
            else:
                print(f"   ⚠️  Aviso: DLL não encontrada após cópia")
                
        except Exception as e:
            print(f"❌ Erro ao copiar DLL para {exe_dir}: {e}")
    
    print("🎯 Script concluído!")
    return 0

if __name__ == "__main__":
    sys.exit(main())