use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    Rgba
};
use std::path::Path;

pub struct MergeOptions {
    pub input: Vec<String>,
    pub output: String,
}

pub fn merge(parts: &[DynamicImage]) -> DynamicImage {
    let first = &parts[0];
    let (width, height) = first.dimensions();
    let mut result = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let mut f_pix = [0u8; 4];
            for part in parts {
                let part = part.get_pixel(x, y).0;
                for i in 0..3 {
                    f_pix[i] = f_pix[i].wrapping_add(part[i]);
                }
            }
            f_pix[3] = 255;
            
            result.put_pixel(x, y, Rgba(f_pix));
        }
    }
    
    DynamicImage::ImageRgba8(result)
}

pub fn run(opts: MergeOptions) -> Result<(), Box<dyn std::error::Error>> {
    if opts.input.is_empty() {
        return Err("No path provided".into());
    }
    let mut n_paths = Vec::new();
    for path_str in &opts.input {
        let path = Path::new(path_str);
        let stem = path.file_stem()
            .ok_or("Invalid filename")?
            .to_str()
            .ok_or("Invalid Unicode in output path")?;
        
        let num = if let Some(last_dot) = stem.rfind('.') {
            stem[last_dot + 1..].parse::<usize>().ok()
        } else {
            None
        };
        
        if let Some(n) = num {
            n_paths.push((n, path_str.clone()));
        }
    }
    n_paths.sort_by_key(|k| k.0);
    for (i, &(num, _)) in n_paths.iter().enumerate() {
        if num != i + 1 {
            return Err(format!("Missing or incorrect numbering of parts. Expected: {}, found: {}", i + 1, num).into());
        }
    }
    let mut parts = Vec::new();
    for (_, path) in &n_paths {
        let image = image::open(path)?;
        parts.push(image);
    }
    if let Some(first) = parts.first() {
        let (width, height) = first.dimensions();
        if parts.iter().any(|img| img.dimensions() != (width, height)) {
            return Err("All partial images must have the same size".into());
        }
    } else {
        return Err("No part images found".into());
    }
    let result = merge(&parts);
    result.save(&opts.output)?;
    
    Ok(())
}