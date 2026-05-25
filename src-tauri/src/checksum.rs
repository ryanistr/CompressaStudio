use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const BUFFER_SIZE: usize = 64 * 1024;

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)
        .with_context(|| format!("Unable to open file for checksum: {}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let digest = hasher.finalize();
    Ok(format!("{digest:x}"))
}
