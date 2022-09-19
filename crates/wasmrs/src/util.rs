pub(crate) fn from_u32_bytes(bytes: &[u8]) -> u32 {
    let mut num_parts: [u8; 4] = Default::default();

    num_parts[0..4].copy_from_slice(bytes);

    u32::from_be_bytes(num_parts)
}

pub(crate) fn from_u24_bytes(bytes: &[u8]) -> u32 {
    let mut num_parts: [u8; 4] = Default::default();

    num_parts[1..4].copy_from_slice(bytes);

    u32::from_be_bytes(num_parts)
}

pub(crate) fn to_u24_bytes(num: u32) -> Vec<u8> {
    let mut num_parts = Vec::with_capacity(3);

    num_parts.extend(&num.to_be_bytes()[1..4]);

    num_parts
}
