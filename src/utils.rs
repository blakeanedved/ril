use std::convert::TryFrom;

pub fn slice_to_u32(slice: &[u8]) -> anyhow::Result<u32> {
    Ok(u32::from_be_bytes(<[u8; 4]>::try_from(slice)?))
}

pub fn slice_to_u16(slice: &[u8]) -> anyhow::Result<u16> {
    Ok(u16::from_be_bytes(<[u8; 2]>::try_from(slice)?))
}
