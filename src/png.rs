use crate::utils::*;
use std::collections::HashMap;

pub struct PngImage {
    pub width: u32,
    pub height: u32,

    // TODO check if these types could be represented by an enum
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,

    pub text_entries: Option<HashMap<String, String>>,
    pub last_changed: Option<String>,
    pub gamma: Option<f32>,
}

#[derive(Debug)]
pub struct PngChunk {
    pub length: usize,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum ColorType {
    Grayscale,
    Truecolor,
    Indexed,
    GrayscaleAlpha,
    TruecolorAlpha,
}

impl From<u8> for ColorType {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Grayscale,      // 000
            2 => Self::Truecolor,      // 010
            3 => Self::Indexed,        // 011
            4 => Self::GrayscaleAlpha, // 100
            6 => Self::TruecolorAlpha, // 110
            _ => unreachable!(),
        }
    }
}

#[allow(non_snake_case)]
fn parse_IHDR_chunk(chunk: &PngChunk) -> anyhow::Result<(u32, u32, u8, ColorType, u8, u8, u8)> {
    let width = slice_to_u32(&chunk.data[0..4])?;
    let height = slice_to_u32(&chunk.data[4..8])?;
    let bit_depth = chunk.data[8];
    let color_type = ColorType::from(chunk.data[9]);
    let compression_method = chunk.data[10];
    let filter_method = chunk.data[11];
    let interlace_method = chunk.data[12];
    Ok((
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    ))
}

#[allow(non_snake_case)]
fn parse_tEXt_chunks(chunks: &Vec<PngChunk>) -> anyhow::Result<HashMap<String, String>> {
    let mut text_entries = HashMap::new();

    for chunk in chunks {
        let mut index: usize = 0;

        while index < chunk.length {
            let start_index = index;
            let key: String;
            loop {
                if chunk.data[index] == 0 {
                    key = String::from_utf8_lossy(&chunk.data[start_index..index]).to_string();
                    break;
                }

                index += 1;
            }
            index += 1;

            let start_index = index;
            let value: String;
            loop {
                if index >= chunk.length || chunk.data[index] == 0 {
                    value = String::from_utf8_lossy(&chunk.data[start_index..index]).to_string();
                    break;
                }

                index += 1;
            }
            index += 1;

            text_entries.insert(key, value);
        }
    }

    Ok(text_entries)
}

#[allow(non_snake_case)]
fn parse_tIME_chunk(chunk: &PngChunk) -> anyhow::Result<String> {
    let year = slice_to_u16(&chunk.data[0..2])?;
    let month = chunk.data[2];
    let day = chunk.data[3];
    let hour = chunk.data[4];
    let minute = chunk.data[5];
    let second = chunk.data[6];

    Ok(format!(
        "{}/{}/{} {}:{:02}:{:02}",
        month, day, year, hour, minute, second
    ))
}

#[allow(non_snake_case)]
fn parse_gAMA_chunk(chunk: &PngChunk) -> anyhow::Result<f32> {
    Ok(slice_to_u32(&chunk.data[0..4])? as f32 / 100000.0)
}

fn read_png_chunks(bytes: &[u8]) -> anyhow::Result<HashMap<String, Vec<PngChunk>>> {
    let mut chunks: HashMap<String, Vec<PngChunk>> = HashMap::new();

    let mut index = 0;
    loop {
        if index >= bytes.len() {
            break;
        }

        let length = slice_to_u32(&bytes[index..index + 4])? as usize;
        index += 4;
        let chunk_type = String::from_utf8_lossy(&bytes[index..index + 4]);
        index += 4;
        let chunk_data = &bytes[index..index + length as usize];
        index += length as usize;
        let _crc = &bytes[index..index + 4];
        index += 4;
        // TODO check CRC

        if chunks.contains_key(&chunk_type.to_string()) {
            chunks
                .get_mut(&chunk_type.to_string())
                .unwrap()
                .push(PngChunk {
                    length,
                    data: chunk_data.to_vec(),
                })
        } else {
            chunks.insert(
                chunk_type.to_string(),
                vec![PngChunk {
                    length,
                    data: chunk_data.to_vec(),
                }],
            );
        }
    }

    Ok(chunks)
}

pub fn read_png(bytes: &[u8]) -> anyhow::Result<PngImage> {
    assert!(&bytes[0..8] == &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]);

    // TODO check if these chunks exists before trying to read them obviously

    let chunks = read_png_chunks(&bytes[8..])?;
    let (width, height, bit_depth, color_type, compression_method, filter_method, interlace_method) =
        parse_IHDR_chunk(&chunks[&"IHDR".to_owned()][0])?;

    let text_entries = if chunks.contains_key(&"tEXT".to_owned()) {
        Some(parse_tEXt_chunks(&chunks[&"tEXt".to_owned()])?)
    } else {
        None
    };

    let last_changed = if chunks.contains_key(&"tIME".to_owned()) {
        Some(parse_tIME_chunk(&chunks[&"tIME".to_owned()][0])?)
    } else {
        None
    };

    let gamma = if chunks.contains_key(&"gAMA".to_owned()) {
        Some(parse_gAMA_chunk(&chunks[&"gAMA".to_owned()][0])?)
    } else {
        None
    };

    // TODO iCCP
    // TODO eXIf
    // TODO iTXt
    // TODO IDAT

    println!("{:?}", chunks.keys());

    println!("bit_depth = {}", bit_depth);
    println!("color_type = {:?}", color_type);

    Ok(PngImage {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
        text_entries,
        last_changed,
        gamma,
    })
}
