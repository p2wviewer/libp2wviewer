use image::{
    DynamicImage,
    GenericImageView,
    ImageBuffer,
    Rgba
};
use rand::Rng;
use std::path::Path;

pub struct SplitOptions {
    pub input_path: String,
    pub num_parts: u32,
    pub delete_original: bool,
}

pub fn split(image: &DynamicImage, num_parts: u32) -> Vec<DynamicImage> {
    let (width, height) = image.dimensions();
    let mut rng = rand::rng();
    let mut parts: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = vec![];
    for _ in 0..num_parts-1 {
        let mut part = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let random_pixel = Rgba([
                    rng.random::<u8>(),
                    rng.random::<u8>(),
                    rng.random::<u8>(),
                    255,
                ]);
                part.put_pixel(x, y, random_pixel);
            }
        }
        parts.push(part);
    }
    let mut f_part = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let original = image.get_pixel(x, y);
            let mut f_pixel = [0u8; 4];
            f_pixel.copy_from_slice(&original.0);
            for part in &parts {
                let part_pixel = part.get_pixel(x, y).0;
                for i in 0..3 {
                    f_pixel[i] = f_pixel[i].wrapping_sub(part_pixel[i]);
                }
            }
            f_pixel[3] = 255;
            
            f_part.put_pixel(x, y, Rgba(f_pixel));
        }
    }
    let mut result = parts.into_iter()
        .map(|img| DynamicImage::ImageRgba8(img))
        .collect::<Vec<_>>();
    result.push(DynamicImage::ImageRgba8(f_part));
    
    result
}

pub fn run(opts: SplitOptions) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(&opts.input_path)?;
    let parts = split(&img, opts.num_parts);
    let path = Path::new(&opts.input_path);
    let output_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let stem = path.file_stem()
        .ok_or("Can't extract filename")?
        .to_str()
        .ok_or("Invalid filename")?;
    let extension = path.extension()
        .ok_or("No Extension")?
        .to_str()
        .ok_or("Invalid Unicode in output path")?;
    for (i, part) in parts.iter().enumerate() {
        let output_filename = format!("{}.{}.{}", stem, i + 1, extension);
        let output_path = output_dir.join(output_filename);
        part.save(&output_path)?;
    }
    if opts.delete_original {
        std::fs::remove_file(&opts.input_path)?;
    }
    Ok(())
}