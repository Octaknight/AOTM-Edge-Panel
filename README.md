# HT32 Panel

[![Release](https://github.com/ananthb/ht32-panel/actions/workflows/release.yml/badge.svg)](https://github.com/ananthb/ht32-panel/actions/workflows/release.yml)

Front-panel display and LED control for mini PCs with HT32-based LCD and RGB LEDs (AceMagic S1S, Skullsaints Agni, etc.).

## Features

- **Daemon** (`ht32paneld`): D-Bus service with HTMX web UI
- **CLI** (`ht32panelctl`): D-Bus client for daemon control
- **Applet**: System tray for GNOME/KDE

## Hardware

| Component | Interface | Details |
|-----------|-----------|---------|
| LCD Display | USB HID | VID:PID 04D9:FD01, 320x170 RGB565 |
| LED Strip | Serial | CH340, 10000 baud |

## Installation

Download the latest release from [GitHub Releases](https://github.com/ananthb/ht32-panel/releases).

### AppImage

```bash
chmod +x ht32-panel-*-x86_64.AppImage
./ht32-panel-*-x86_64.AppImage
```

### Linux Binary

```bash
tar -xzf ht32-panel-*-x86_64-linux.tar.gz
./ht32paneld config/default.toml
```

### NixOS

```nix
{
  inputs.ht32-panel.url = "github:ananthb/ht32-panel";
}
```

```nix
{ inputs, ... }:
{
  imports = [ inputs.ht32-panel.nixosModules.default ];

  services.ht32-panel = {
    enable = true;
    led.theme = 2;  # breathing
  };
}
```

### Nix

```bash
nix run github:ananthb/ht32-panel
```

### From Source

```bash
git clone https://github.com/ananthb/ht32-panel
cd ht32-panel
cargo build --release
```

## Usage

```bash
# Start daemon
ht32paneld config/default.toml

# CLI (requires daemon)
ht32panelctl lcd orientation landscape
ht32panelctl led set rainbow --intensity 3 --speed 3
```

To enable the web UI, set `web.enable = true` in the config file.

## D-Bus

The daemon exposes `org.ht32panel.Daemon1` on the session bus.

## Acknowledgement

My thanks for the ideas and source code from [github.com/tjaworski/AceMagic-S1-LED-TFT-Linux](https://github.com/tjaworski/AceMagic-S1-LED-TFT-Linux/commit/2971f2b0703bd3170a3f714867652f7e085ec447).

## License

ht32-panel is licensed under the terms of the AGPL license.
See [LICENSE](LICENSE) for the full license text.

Copyright &#169; Ananth Bhaskararaman 2026
