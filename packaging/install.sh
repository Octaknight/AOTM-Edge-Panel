#!/bin/bash
set -e

if [[ $EUID -ne 0 ]]; then
   echo "This script must be run as root" 
   exit 1
fi

echo "Installing HT32 Panel..."

# Install binary
install -m 755 ht32paneld /usr/local/bin/

# Install config
mkdir -p /etc/ht32-panel
CONFIG_SRC="config.toml"

# Check if we are running from extracted tarball structure
if [ -f config/default.toml ]; then
    CONFIG_SRC="config/default.toml"
elif [ -f default.toml ]; then
    CONFIG_SRC="default.toml"
fi

if [ ! -f /etc/ht32-panel/config.toml ]; then
    if [ -f "$CONFIG_SRC" ]; then
        install -m 644 "$CONFIG_SRC" /etc/ht32-panel/config.toml
        echo "Installed default config to /etc/ht32-panel/config.toml"
    else
        echo "Warning: config file not found ($CONFIG_SRC), skipping config installation"
    fi
else
    echo "Config already exists at /etc/ht32-panel/config.toml, skipping overwrite"
fi

# Install Wallpaper
mkdir -p /usr/share/ht32-panel
if [ -f octaknight-wallpaper.png ]; then
    install -m 644 octaknight-wallpaper.png /usr/share/ht32-panel/
    echo "Installed wallpaper to /usr/share/ht32-panel/"
fi

# Install udev rules
install -m 644 99-ht32-panel.rules /etc/udev/rules.d/
udevadm control --reload-rules && udevadm trigger

# Install systemd service
install -m 644 ht32-panel.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable ht32-panel.service
systemctl start ht32-panel.service

echo "Installation complete!"
echo "Service status:"
systemctl status ht32-panel.service --no-pager
