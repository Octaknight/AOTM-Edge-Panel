//! Image face that displays a user-selected image.

use super::{Complication, ComplicationOption, EnabledComplications, Face, Theme};
use crate::rendering::Canvas;
use crate::sensors::data::SystemData;
use std::sync::Mutex;
use tiny_skia::{Pixmap, PixmapPaint, Transform};
use tracing::{error, info};

/// Face that displays a static image.
pub struct ImageFace {
    /// Cached image data: (path, pixmap, timestamp of last load)
    cache: Mutex<Option<(String, Pixmap)>>,
}

impl ImageFace {
    /// Creates a new Image face.
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(None),
        }
    }

    /// Loads an image from path and resizes it to fit the canvas.
    fn load_image(&self, path: &str, width: u32, height: u32) -> Option<Pixmap> {
        info!("Loading image from: {}", path);
        match image::open(path) {
            Ok(img) => {
                // Resize to fit canvas while preserving aspect ratio
                let img = img.resize(width, height, image::imageops::FilterType::Lanczos3);
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                
                let mut data = rgba.into_raw();
                
                // Premultiply alpha for tiny-skia
                for chunk in data.chunks_mut(4) {
                    let a = chunk[3] as u32;
                    if a != 255 {
                        chunk[0] = ((chunk[0] as u32 * a) / 255) as u8;
                        chunk[1] = ((chunk[1] as u32 * a) / 255) as u8;
                        chunk[2] = ((chunk[2] as u32 * a) / 255) as u8;
                    }
                }

                if let Some(size) = tiny_skia::IntSize::from_wh(w, h) {
                    return Some(Pixmap::from_vec(data, size).unwrap());
                } else {
                    return None;
                }
            }
            Err(e) => {
                error!("Failed to load image: {}", e);
                None
            }
        }
    }
}

impl Face for ImageFace {
    fn name(&self) -> &str {
        "image"
    }

    fn available_complications(&self) -> Vec<Complication> {
        vec![Complication::with_options(
            "settings",
            "Settings",
            "Image configuration",
            true,
            vec![ComplicationOption::text(
                "path",
                "Image Path",
                "Absolute path to the image file",
                "",
            )],
        )]
    }

    fn render(
        &self,
        canvas: &mut Canvas,
        _data: &SystemData,
        theme: &Theme,
        complications: &EnabledComplications,
    ) {
        canvas.clear();

        // Get configured path
        let path_opt = complications
            .get_option(self.name(), "settings", "path");
            
        let path = if let Some(p) = path_opt {
            p.clone()
        } else {
            String::new()
        };

        if path.is_empty() {
            canvas.draw_text(10, 10, "No image configured.", 16.0, theme.text);
            canvas.draw_text(10, 30, "Set path in settings.", 14.0, theme.text);
            return;
        }

        let (cw, ch) = canvas.dimensions();
        let mut cache = self.cache.lock().unwrap();

        // Reload if path changed or no cache
        let should_reload = match &*cache {
            Some((cached_path, _)) => cached_path != &path,
            None => true,
        };

        if should_reload {
            if let Some(pixmap) = self.load_image(&path, cw, ch) {
                *cache = Some((path.clone(), pixmap));
            } else {
                // If failed to load, clear cache to avoid stuck state if file is fixed later?
                // Or maybe keep old image if path changed but new one is invalid?
                // For now, clear cache to force trying again or show error
                *cache = None;
            }
        }

        // Draw cached image
        if let Some((_, pixmap)) = &*cache {
            // center the image
            let x = (cw as i32 - pixmap.width() as i32) / 2;
            let y = (ch as i32 - pixmap.height() as i32) / 2;
            canvas.draw_pixmap(x, y, pixmap);
        } else {
            canvas.draw_text(10, 10, "Failed to load image.", 16.0, 0xFF0000); // Red error
            canvas.draw_text(10, 30, &path, 12.0, theme.text);
        }
    }
}
