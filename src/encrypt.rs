use crate::crypto::{derive_key_from_password, derive_key_from_file, generate_nonce, generate_token, encrypt};
use crate::header::Header;
use crate::image_ops::{load_image, save_image, itb, bti};
use anyhow::{Result, bail};
use log::info;
use std::path::{Path, PathBuf};

pub struct EncryptOptions {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub password: Option<String>,
    pub password_file: Option<String>,
    pub split: Option<u32>,
}

pub fn run(opts: EncryptOptions) -> Result<()> {
    info!("Starting encryption for {:?}", opts.input_path);

    // load image
    let img = load_image(&opts.input_path)?;
    let img_bytes = itb(&img);
    let (original_width, original_height) = (img.width(), img.height());

    // Derive key
    let (key, salt) = if let Some(ref pw_file) = opts.password_file {
        derive_key_from_file(pw_file)?
    } else if let Some(ref pw) = opts.password {
        derive_key_from_password(pw, None)? // generate random salt
    } else {
        bail!("No password or password file provided");
    };

    let nonce = generate_nonce();
    let token = generate_token(&key);

    let mut payload = token.to_vec();
    payload.extend_from_slice(&img_bytes);
    let (ciphertext, auth_tag) = encrypt(&key, &nonce, &payload)?;

    let payload_len = (crate::header::HEADER_SIZE + ciphertext.len()) as u64;
    let header = Header::new(1, nonce, auth_tag, salt, original_width, original_height, payload_len);
    let header_bytes = header.to_bytes();
    
    let mut final_bytes = Vec::with_capacity(payload_len as usize);
    final_bytes.extend_from_slice(&header_bytes);
    final_bytes.extend_from_slice(&ciphertext);

    let splits = opts.split.unwrap_or(1);
    if splits <= 1 {
        // save as a single image
        let pixels_needed = (final_bytes.len() as u32 + 3) / 4;
        let new_width = original_width;
        let new_height = (pixels_needed + new_width - 1) / new_width;
        let new_image_size = (new_width * new_height * 4) as usize;

        let mut padded_final_bytes = final_bytes;
        padded_final_bytes.resize(new_image_size, 0);

        info!("Creating single image of dimensions {}x{}", new_width, new_height);
        let final_img = bti(&padded_final_bytes, new_width, new_height)?;
        save_image(&final_img, &opts.output_path)?;
    } else {
        // split into multiple images
        let chunk_size = (final_bytes.len() + splits as usize - 1) / splits as usize;
        let output_dir = opts.output_path.parent().unwrap_or_else(|| Path::new("."));
        let output_stem = opts.output_path.file_stem().unwrap_or_default().to_str().unwrap();
        let output_ext = opts.output_path.extension().unwrap_or_default().to_str().unwrap_or("png");

        for i in 0..splits {
            let start = i as usize * chunk_size;
            let end = std::cmp::min(start + chunk_size, final_bytes.len());
            if start >= end { continue; }
            let chunk = &final_bytes[start..end];

            let width = original_width;
            let pixels_needed = (chunk.len() as u32 + 3) / 4;
            let new_height = (pixels_needed + width - 1) / width;
            let new_image_size = (width * new_height * 4) as usize;

            let mut padded_chunk = chunk.to_vec();
            padded_chunk.resize(new_image_size, 0);

            let split_img = bti(&padded_chunk, width, new_height)?;
            let path = output_dir.join(format!(
                "{}.{}.{}",
                output_stem,
                i + 1,
                output_ext
            ));
            info!("Saving chunk {} to {:?}", i + 1, path);
            save_image(&split_img, &path)?;
        }
    }
    info!("Encryption completed successfully.");
    Ok(())
}
