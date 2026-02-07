# ht32-panel-client

D-Bus client library for communicating with the HT32 Panel daemon.

## Usage

```rust
use ht32_panel_client::DaemonClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = DaemonClient::new().await?;

    // Get current status
    let status = client.status().await?;

    // Control LCD
    client.set_orientation("landscape").await?;

    // Control LEDs
    client.set_led_effect("rainbow", 3, 3).await?;

    Ok(())
}
```

## D-Bus Interface

Connects to `org.ht32panel.Daemon1` on either the system or session bus.

## License

AGPL-3.0-or-later
