//! D-Bus client library for communicating with the HT32 Panel Daemon.
//!
//! This crate provides a unified client for both CLI and applet use cases.

use anyhow::{Context, Result};
use tracing::debug;
use zbus::{proxy, Connection};

/// D-Bus bus type selection.
#[derive(Debug, Clone, Copy, Default)]
pub enum BusType {
    /// Session bus (user session).
    Session,
    /// System bus (system-wide).
    System,
    /// Try session first, fall back to system.
    #[default]
    Auto,
}

/// D-Bus proxy for the HT32 Panel Daemon.
#[proxy(
    interface = "org.ht32panel.Daemon1",
    default_service = "org.ht32panel.Daemon",
    default_path = "/org/ht32panel/Daemon"
)]
trait Daemon1 {
    /// Sets the display orientation.
    fn set_orientation(&self, orientation: &str) -> zbus::Result<()>;

    /// Gets the current orientation.
    fn get_orientation(&self) -> zbus::Result<String>;

    /// Clears the display to a solid color.
    fn clear_display(&self, color: &str) -> zbus::Result<()>;

    /// Sets the display face.
    fn set_face(&self, face: &str) -> zbus::Result<()>;

    /// Gets the current face name.
    fn get_face(&self) -> zbus::Result<String>;

    /// Sets LED parameters.
    fn set_led(&self, theme: u8, intensity: u8, speed: u8) -> zbus::Result<()>;

    /// Turns off LEDs.
    fn led_off(&self) -> zbus::Result<()>;

    /// Gets current LED settings as (theme, intensity, speed).
    fn get_led_settings(&self) -> zbus::Result<(u8, u8, u8)>;

    /// Gets the background color as hex string.
    fn get_background_color(&self) -> zbus::Result<String>;

    /// Sets the background color from hex string.
    fn set_background_color(&self, color: &str) -> zbus::Result<()>;

    /// Gets the foreground color as hex string.
    fn get_foreground_color(&self) -> zbus::Result<String>;

    /// Sets the foreground color from hex string.
    fn set_foreground_color(&self, color: &str) -> zbus::Result<()>;

    /// Gets the background image path.
    fn get_background_image(&self) -> zbus::Result<String>;

    /// Sets the background image path.
    fn set_background_image(&self, path: &str) -> zbus::Result<()>;

    /// Clears the background image.
    fn clear_background_image(&self) -> zbus::Result<()>;

    /// Gets the refresh rate in seconds.
    fn get_refresh_rate(&self) -> zbus::Result<u32>;

    /// Sets the refresh rate in seconds.
    fn set_refresh_rate(&self, secs: u32) -> zbus::Result<()>;

    /// Gets the current network interface.
    fn get_network_interface(&self) -> zbus::Result<String>;

    /// Sets the network interface to monitor.
    fn set_network_interface(&self, interface: &str) -> zbus::Result<()>;

    /// Lists all available network interfaces.
    fn list_network_interfaces(&self) -> zbus::Result<Vec<String>>;

    /// Returns the current framebuffer as PNG data.
    fn get_screen_png(&self) -> zbus::Result<Vec<u8>>;

    /// Shuts down the daemon.
    fn quit(&self) -> zbus::Result<()>;

    /// Whether the LCD device is connected.
    #[zbus(property)]
    fn connected(&self) -> zbus::Result<bool>;

    /// Whether the web UI is enabled.
    #[zbus(property)]
    fn web_enabled(&self) -> zbus::Result<bool>;

    /// Current display orientation.
    #[zbus(property)]
    fn orientation(&self) -> zbus::Result<String>;

    /// Current LED theme (1-5).
    #[zbus(property)]
    fn led_theme(&self) -> zbus::Result<u8>;

    /// Current LED intensity (1-5).
    #[zbus(property)]
    fn led_intensity(&self) -> zbus::Result<u8>;

    /// Current LED speed (1-5).
    #[zbus(property)]
    fn led_speed(&self) -> zbus::Result<u8>;

    /// Current refresh rate in seconds.
    #[zbus(property)]
    fn refresh_rate(&self) -> zbus::Result<u32>;

    /// Current network interface name.
    #[zbus(property)]
    fn network_interface(&self) -> zbus::Result<String>;

    /// Current display face name.
    #[zbus(property)]
    fn face(&self) -> zbus::Result<String>;
}

/// D-Bus client wrapper for the daemon.
pub struct DaemonClient {
    proxy: Daemon1Proxy<'static>,
}

impl DaemonClient {
    /// Attempts to connect to the daemon via D-Bus with auto bus detection.
    ///
    /// Tries session bus first, falls back to system bus.
    pub async fn connect() -> Result<Self> {
        Self::connect_with_bus(BusType::Auto).await
    }

    /// Attempts to connect to the daemon via D-Bus with specified bus type.
    pub async fn connect_with_bus(bus_type: BusType) -> Result<Self> {
        let connection = match bus_type {
            BusType::Session => {
                debug!("Connecting to session bus");
                Connection::session()
                    .await
                    .context("Failed to connect to session bus")?
            }
            BusType::System => {
                debug!("Connecting to system bus");
                Connection::system()
                    .await
                    .context("Failed to connect to system bus")?
            }
            BusType::Auto => match Connection::session().await {
                Ok(conn) => {
                    debug!("Connected to session bus");
                    conn
                }
                Err(session_err) => {
                    debug!(
                        "Session bus unavailable ({}), trying system bus",
                        session_err
                    );
                    Connection::system()
                        .await
                        .context("Failed to connect to any D-Bus")?
                }
            },
        };

        let proxy = Daemon1Proxy::new(&connection)
            .await
            .context("Failed to create D-Bus proxy")?;

        Ok(Self { proxy })
    }

    /// Sets the display orientation.
    pub async fn set_orientation(&self, orientation: &str) -> Result<()> {
        self.proxy
            .set_orientation(orientation)
            .await
            .context("Failed to set orientation via D-Bus")
    }

    /// Gets the current orientation.
    pub async fn get_orientation(&self) -> Result<String> {
        self.proxy
            .get_orientation()
            .await
            .context("Failed to get orientation via D-Bus")
    }

    /// Clears the display to a solid color.
    pub async fn clear_display(&self, color: &str) -> Result<()> {
        self.proxy
            .clear_display(color)
            .await
            .context("Failed to clear display via D-Bus")
    }

    /// Sets the display face.
    pub async fn set_face(&self, face: &str) -> Result<()> {
        self.proxy
            .set_face(face)
            .await
            .context("Failed to set face via D-Bus")
    }

    /// Gets the current face name.
    pub async fn get_face(&self) -> Result<String> {
        self.proxy
            .get_face()
            .await
            .context("Failed to get face via D-Bus")
    }

    /// Sets LED parameters.
    pub async fn set_led(&self, theme: u8, intensity: u8, speed: u8) -> Result<()> {
        self.proxy
            .set_led(theme, intensity, speed)
            .await
            .context("Failed to set LED via D-Bus")
    }

    /// Turns off LEDs.
    pub async fn led_off(&self) -> Result<()> {
        self.proxy
            .led_off()
            .await
            .context("Failed to turn off LED via D-Bus")
    }

    /// Gets current LED settings.
    pub async fn get_led_settings(&self) -> Result<(u8, u8, u8)> {
        self.proxy
            .get_led_settings()
            .await
            .context("Failed to get LED settings via D-Bus")
    }

    /// Gets the background color as hex string.
    pub async fn get_background_color(&self) -> Result<String> {
        self.proxy
            .get_background_color()
            .await
            .context("Failed to get background color via D-Bus")
    }

    /// Sets the background color from hex string.
    pub async fn set_background_color(&self, color: &str) -> Result<()> {
        self.proxy
            .set_background_color(color)
            .await
            .context("Failed to set background color via D-Bus")
    }

    /// Gets the foreground color as hex string.
    pub async fn get_foreground_color(&self) -> Result<String> {
        self.proxy
            .get_foreground_color()
            .await
            .context("Failed to get foreground color via D-Bus")
    }

    /// Sets the foreground color from hex string.
    pub async fn set_foreground_color(&self, color: &str) -> Result<()> {
        self.proxy
            .set_foreground_color(color)
            .await
            .context("Failed to set foreground color via D-Bus")
    }

    /// Gets the background image path.
    pub async fn get_background_image(&self) -> Result<String> {
        self.proxy
            .get_background_image()
            .await
            .context("Failed to get background image via D-Bus")
    }

    /// Sets the background image path.
    pub async fn set_background_image(&self, path: &str) -> Result<()> {
        self.proxy
            .set_background_image(path)
            .await
            .context("Failed to set background image via D-Bus")
    }

    /// Clears the background image.
    pub async fn clear_background_image(&self) -> Result<()> {
        self.proxy
            .clear_background_image()
            .await
            .context("Failed to clear background image via D-Bus")
    }

    /// Gets the refresh rate in seconds.
    pub async fn get_refresh_rate(&self) -> Result<u32> {
        self.proxy
            .get_refresh_rate()
            .await
            .context("Failed to get refresh rate via D-Bus")
    }

    /// Sets the refresh rate in seconds.
    pub async fn set_refresh_rate(&self, secs: u32) -> Result<()> {
        self.proxy
            .set_refresh_rate(secs)
            .await
            .context("Failed to set refresh rate via D-Bus")
    }

    /// Gets the current network interface.
    pub async fn get_network_interface(&self) -> Result<String> {
        self.proxy
            .get_network_interface()
            .await
            .context("Failed to get network interface via D-Bus")
    }

    /// Sets the network interface.
    pub async fn set_network_interface(&self, interface: &str) -> Result<()> {
        self.proxy
            .set_network_interface(interface)
            .await
            .context("Failed to set network interface via D-Bus")
    }

    /// Lists available network interfaces.
    pub async fn list_network_interfaces(&self) -> Result<Vec<String>> {
        self.proxy
            .list_network_interfaces()
            .await
            .context("Failed to list network interfaces via D-Bus")
    }

    /// Gets the screen as PNG data.
    pub async fn get_screen_png(&self) -> Result<Vec<u8>> {
        self.proxy
            .get_screen_png()
            .await
            .context("Failed to get screen PNG via D-Bus")
    }

    /// Shuts down the daemon.
    pub async fn quit(&self) -> Result<()> {
        self.proxy
            .quit()
            .await
            .context("Failed to quit daemon via D-Bus")
    }

    /// Checks if the LCD is connected.
    pub async fn is_connected(&self) -> Result<bool> {
        self.proxy
            .connected()
            .await
            .context("Failed to get connection status via D-Bus")
    }

    /// Checks if the web UI is enabled.
    pub async fn is_web_enabled(&self) -> Result<bool> {
        self.proxy
            .web_enabled()
            .await
            .context("Failed to get web enabled status via D-Bus")
    }
}
