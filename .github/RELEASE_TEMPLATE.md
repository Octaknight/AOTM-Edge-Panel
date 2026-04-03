## Installation

1.  **Download** the `ht32-panel-*-x86_64-linux.tar.gz` file from the assets below.
2.  **Extract** the archive and enter the extracted directory:
    ```bash
    tar -xzf ht32-panel-*-x86_64-linux.tar.gz
    cd ht32-panel-*-x86_64-linux
    ```
3.  **Run the checks**:
    ```bash
    # (Optional) Verify the display is connected
    lsusb -d 04D9:FD01
    ```
4.  **Install**:
    ```bash
    sudo ./install.sh
    ```

This installs the daemon and CLI into `/usr/local/bin`, writes the config to `/etc/ht32-panel/config.toml`, installs the systemd service and udev rules, and copies the default wallpaper into `/usr/share/ht32-panel/`.

The service starts automatically. You can check status and logs with:
```bash
systemctl status ht32-panel
journalctl -u ht32-panel -n 50 --no-pager
```

If the web UI is enabled in the config, open:

- `http://localhost:8686/` on the device itself
- `http://<device-ip>:8686/` from another machine on the same network
