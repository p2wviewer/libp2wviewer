use crate::crypto::{
    pwd2key,
    file2key,
    decrypt,
    generate_token
};
use crate::header::Header;
use crate::image_ops::{
    load,
    save,
    img2byte,
    byte2img
};
use anyhow::{
    Result,
    bail
};
use log::{
    info,
    debug
};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub struct DecryptOptions {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub password: Option<String>,
    pub password_file: Option<String>,
}

fn image_bytes(path: &PathBuf) -> Result<Vec<u8>> {
    if path.is_dir() {
        info!("Input is a directory, searching for split parts...");
        let re = Regex::new(r"\.(\d+)\.(png|PNG)$").unwrap();
        let mut files_with_parts: Vec<(u32, PathBuf)> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_file() {
                if let Some(filename) = p.file_name().and_then(|f| f.to_str()) {
                    if let Some(captures) = re.captures(filename) {
                        if let Some(part_num_str) = captures.get(1) {
                            if let Ok(part_num) = part_num_str.as_str().parse::<u32>() {
                                files_with_parts.push((part_num, p.clone()));
                            }
                        }
                    }
                }
            }
        }

        if files_with_parts.is_empty() {
            bail!("No split image parts found in the directory: {:?}", path);
        }

        // Sort by part number
        files_with_parts.sort_by_key(|k| k.0);

        let first_path = files_with_parts[0].1.clone();
        let first_img = load(&first_path)?;
        let first_part_bytes = img2byte(&first_img);
        if first_part_bytes.len() < crate::header::HEADER_SIZE {
            bail!("First part image is too small to contain a header");
        }
        let header = Header::from_bytes(&first_part_bytes[..crate::header::HEADER_SIZE])?;
        let total_len = header.payload_len as usize;
        debug!("Expecting {} bytes based on header in part 1", total_len);

        let num_parts = files_with_parts.len();
        let chunk_size = (total_len + num_parts - 1) / num_parts;
        let last_chunk_size = total_len - (num_parts - 1) * chunk_size;

        let mut f_bytes = first_part_bytes[0 .. std::cmp::min(chunk_size, first_part_bytes.len())].to_vec();
        if f_bytes.len() != chunk_size {
            bail!("First part doesn't contain enough data: {} < {}", f_bytes.len(), chunk_size);
        }
        debug!("Read {} bytes from part 1, total is now {}", f_bytes.len(), f_bytes.len());

        for i in 1..num_parts {
            let file_path = &files_with_parts[i].1;
            let part_img = load(file_path)?;
            let part_bytes = img2byte(&part_img);
            let this_chunk_size = if i == num_parts - 1 { last_chunk_size } else { chunk_size };
            if part_bytes.len() < this_chunk_size {
                bail!("Part {} too small: {} < {}", i + 1, part_bytes.len(), this_chunk_size);
            }
            f_bytes.extend_from_slice(&part_bytes[0 .. this_chunk_size]);
            debug!("Read {} bytes from part {}, total is now {}", this_chunk_size, i + 1, f_bytes.len());
        }

        if f_bytes.len() != total_len {
            bail!("Failed to reconstruct data. Expected {} bytes, got {}.", total_len, f_bytes.len());
        }

        Ok(f_bytes)

    } else {
        info!("Input is a single file.");
        let mut img_bytes = img2byte(&load(path)?);
        if img_bytes.len() < crate::header::HEADER_SIZE {
            bail!("Input data too small to contain header");
        }
        let header = Header::from_bytes(&img_bytes[..crate::header::HEADER_SIZE])?;
        let expected_len = header.payload_len as usize;
        if img_bytes.len() < expected_len {
            bail!("Input image too small: {} < {}", img_bytes.len(), expected_len);
        }
        img_bytes.truncate(expected_len);
        Ok(img_bytes)
    }
}

pub fn run(opts: DecryptOptions) -> Result<()> {
    info!("Starting decryption for {:?}", opts.input_path);
    let img_bytes = image_bytes(&opts.input_path)?;

    let header = Header::from_bytes(&img_bytes[..crate::header::HEADER_SIZE])?;
    debug!("Parsed header: {:?}", header);
    
    // after header is the ciphertext alles
    let ciphertext = &img_bytes[crate::header::HEADER_SIZE..];
    let auth_tag = &header.auth_tag;

    // Derive key
    let key: [u8; 32] = if let Some(ref pw_file) = opts.password_file {
        file2key(pw_file)?.0
    } else if let Some(ref pw) = opts.password {
        let (k, _salt) = pwd2key(pw, Some(header.salt))?;
        k
    } else {
        bail!("No password or password file is provided");
    };

    let plaintext = decrypt(&key, &header.nonce, ciphertext, auth_tag)?;

    // Verify token
    if plaintext.len() < 32 {
        bail!("Decrypted data too small for token check");
    }
    let token_stored = &plaintext[..32];
    let img_data = &plaintext[32..];
    let token_check = generate_token(&key);
    if token_stored != token_check {
        bail!("Invalid password or corrupted data. Self-check failed.");
    }
    info!("Self-check passed, key is valid");
    // Schon im Header
    let width = header.width;
    let height = header.height;
    let decrypted_img = byte2img(img_data, width, height)?;
    save(&decrypted_img, &opts.output_path)?;

    info!("Decryption completed successfully: {:?}", opts.output_path);
    Ok(())
}
