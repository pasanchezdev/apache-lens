#!/bin/sh

BINARY_PATH="/usr/local/bin/applogs"
CONFIG_DIR="$HOME/.config/applogs"

echo ""
echo "applogs uninstaller"
echo "-------------------"
echo ""
echo "This will remove:"
echo "  $BINARY_PATH"
echo "  $CONFIG_DIR"
echo ""
echo "All configuration and saved settings will be permanently deleted."
echo ""
printf "Are you sure you want to continue? [y/N] "
read -r answer

case "$answer" in
    y|Y)
        ;;
    *)
        echo ""
        echo "Uninstall cancelled."
        echo ""
        exit 0
        ;;
esac

echo ""

if [ -f "$BINARY_PATH" ]; then
    if [ -w "$BINARY_PATH" ]; then
        rm -f "$BINARY_PATH"
    else
        sudo rm -f "$BINARY_PATH"
    fi
    echo "  Removed $BINARY_PATH"
else
    echo "  Binary not found at $BINARY_PATH, skipping."
fi

if [ -d "$CONFIG_DIR" ]; then
    rm -rf "$CONFIG_DIR"
    echo "  Removed $CONFIG_DIR"
else
    echo "  Config directory not found, skipping."
fi

echo ""
echo "applogs has been completely removed from your system."
echo ""
