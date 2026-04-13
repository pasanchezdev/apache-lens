#!/bin/sh

set -e

REPO="https://github.com/pasanchezdev/apache-lens"
BINARY="applogs"
INSTALL_DIR="/usr/local/bin"

echo ""
echo "applogs installer"
echo "-----------------"

if ! command -v cargo > /dev/null 2>&1; then
    echo "Rust no está instalado. Instalando..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --quiet
    . "$HOME/.cargo/env"
    echo "Rust instalado."
fi

if ! command -v git > /dev/null 2>&1; then
    echo "Error: git es necesario para continuar. Instálalo y vuelve a ejecutar este script."
    exit 1
fi

TMP_DIR=$(mktemp -d)
echo "Descargando applogs..."
git clone --quiet "$REPO" "$TMP_DIR/apache-lens"

echo "Compilando..."
cd "$TMP_DIR/apache-lens"
cargo build --release --quiet

echo "Instalando binario en $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
else
    sudo cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
fi

cd /
rm -rf "$TMP_DIR"

echo ""
echo "applogs instalado correctamente."
echo ""

applogs init
