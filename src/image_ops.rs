use anyhow::{Result, bail};
use image::{DynamicImage, ImageBuffer, Rgba};
use log::{debug, info, warn};
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
    info!("Loading image from {:?}", path.as_ref());
    let img = image::open(&path).map_err(|e| {
        warn!("Failed to open image: {}", e);
        e
    })?;
    info!("Image loaded ({}x{})", img.width(), img.height());
    Ok(img)
}

pub fn save_image<P: AsRef<Path>>(img: &DynamicImage, path: P) -> Result<()> {
    info!("Saving image to {:?}", path.as_ref());
    img.save(&path).map_err(|e| {
        warn!("Failed to save image: {}", e);
        e
    })?;
    info!("Image saved");
    Ok(())
}

/// Byte zu Image
/// Expects `data.len() % 4 == 0`
pub fn bti(data: &[u8], width: u32, height: u32) -> Result<DynamicImage> {
    if data.len() != (width * height * 4) as usize {
        bail!("Data length {} does not match dimensions {}x{} (expected {})",
            data.len(),
            width,
            height,
            width * height * 4
        );
    }
    debug!("Converting {} bytes to image {}x{}", data.len(), width, height);
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, data.to_vec())
        .ok_or_else(|| anyhow::anyhow!("Failed to create image Buf from raw bytes"))?;
    Ok(DynamicImage::ImageRgba8(img))
}

/// image to byte
pub fn itb(img: &DynamicImage) -> Vec<u8> {
    let rgba = img.to_rgba8();
    let buf = rgba.into_raw();
    debug!("Converted image {}x{} to {} bytes", img.width(), img.height(), buf.len());
    buf
}