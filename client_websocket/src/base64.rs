const LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P','Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
const PADDING: char = '=';

//&str to &[u8]
pub fn base64_encode(data: &[u8]) -> String {
    //let byte_array: &[u8] = data.as_bytes();
    let mut v: Vec<char> = Vec::new();
    for octet_array in data.chunks(3) {
        v.extend(encode_chunks(octet_array));
    }
    return v.into_iter().collect::<String>();
}

fn encode_chunks(chunks: &[u8]) -> Vec<char> {
    let mut v = Vec::new();
    match chunks.len() {
        3 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[0] & 0b00000011) << 4) | chunks[1] >> 4) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[1] & 0b00001111) << 2) | ((chunks[2] & 0b11000000) >> 6)) as usize]);
            v.push(LOOKUP_TABLE[(chunks[2] & 0b00111111) as usize]);
        },
        2 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[0] & 0b00000011) << 4) | chunks[1] >> 4) as usize]);
            v.push(LOOKUP_TABLE[((chunks[1] & 0b00001111) << 2) as usize]);
            v.push(PADDING);
        },
        1 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[((chunks[0] & 0b00000011) << 4) as usize]);
            v.push(PADDING);
            v.push(PADDING);
        },
        _ => {}
    }
    v
}