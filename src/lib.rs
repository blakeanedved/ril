pub mod png;
#[cfg(test)]
mod tests;
mod utils;

use std::fs;
use std::path::Path;

pub fn read_image_bytes(filename: impl AsRef<Path>) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(filename.as_ref())?)
}
