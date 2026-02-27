## Installation

1.  **Download** the `ht32-panel-*-x86_64-linux.tar.gz` file from the assets below.
2.  **Extract** the archive:
    ```bash
    tar -xzf ht32-panel-*-x86_64-linux.tar.gz
    cd ht32-panel-*
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

This will install the daemon, configure the systemd service, set up USB permissions, and install the default wallpaper.

The service will start automatically. You can check the status with:
```bash
systemctl status ht32-panel
```
