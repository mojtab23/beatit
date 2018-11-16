use super::ID3v2Header;
use super::un_synchsafe;

pub fn parse_3(bytes: &[u8], header: &ID3v2Header) {
    read_frames(bytes);
}

fn read_frames(bytes: &[u8]) {}

fn read_frame_header(bytes: &[u8]) {
    let id_bytes = &[bytes[0], bytes[1], bytes[2], bytes[3]];
    let frame_id = parse_frame_id(id_bytes);
    println!("frame id:{}", frame_id);
}

fn read_extended_header(bytes: &[u8], position: usize) {
    let size_bytes = &[bytes[position + 0], bytes[position + 1], bytes[position + 2], bytes[position + 3]];
    let size = read_header_size(size_bytes).unwrap();
    let flag_first_byte = bytes[position + 4];
    let crc_flag;
//        validate flag
    if bytes[position + 4] & 0b01111111 == 0 && bytes[position + 5] == 0 {
//        CRC flag is set
        if bytes[position + 4] != 0 {
//       todo read CRC 4 bytes
        }
    }

}

fn read_header_size(size_bytes: &[u8; 4]) -> Result<u32, ()> {
    return un_synchsafe(size_bytes);
}

fn parse_frame_id(id_bytes: &[u8; 4]) -> Result<&str, ()> {
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
        && id_bytes[2] <= b'9'
        && id_bytes[3] >= b'A'
        && id_bytes[3] <= b'Z'
        && id_bytes[3] >= b'0'
        && id_bytes[3] <= b'9' {
        let s = format!("{}{}{}", id_bytes[0], id_bytes[1], id_bytes[2], id_bytes[3]);
        return Ok(s.as_str());
    }
    return Err(());
}

fn read_frame_size(id_bytes: &[u8; 4]) {}