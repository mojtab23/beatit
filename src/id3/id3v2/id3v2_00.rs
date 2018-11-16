use super::ID3v2Header;

pub fn parse_00(bytes: &[u8], header: &ID3v2Header) {
    read_frames(bytes);
}

fn read_frames(bytes: &[u8]) {}

fn read_frame_header(bytes: &[u8]) {
    let id_bytes = &[bytes[0], bytes[1], bytes[2]];
    let frame_id = parse_frame_id(id_bytes);
    println!("frame id:{}", frame_id);
}

fn parse_frame_id(id_bytes: &[u8; 3]) -> Result<&str, ()> {
    if id_bytes[0] >= b'A'
        && id_bytes[0] <= b'Z'
        && id_bytes[0] >= b'0'
        && id_bytes[0] <= b'9'
        && id_bytes[1] >= b'A'
        && id_bytes[1] <= b'Z'
        && id_bytes[1] >= b'0'
        && id_bytes[1] <= b'9'
        && id_bytes[2] >= b'A'
        && id_bytes[2] <= b'Z'
        && id_bytes[2] >= b'0'
        && id_bytes[2] <= b'9' {
        let s = format!("{}{}{}", id_bytes[0], id_bytes[1], id_bytes[2]);
        return Ok(s.as_str());
    }
    return Err(());
}