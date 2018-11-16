use id3::id3v2::id3v2_00::parse_00;
use id3::read_string_from_byte_array;

mod id3v2_00;
mod id3v2_3_0;

const HEADER_LENGTH: usize = 10;
const MAJOR_VERSION_OFFSET: usize = 3;
const MINOR_VERSION_OFFSET: usize = 4;
const FLAGS_OFFSET: usize = 5;
const DATA_LENGTH_OFFSET: usize = 6;
//ID3v2/file identifier
const TAG: &str = "ID3";


enum ID3v2Version {
    Id3v2_00,
    Id3v2_3_0,
    Id3v2_4_0,
}

struct ID3v2Header {
    version: ID3v2Version,
    flags: u8,
    size: u32,
}

pub fn parse_id3v2(bytes: &Vec<u8>) -> Option<()> {
    let header_option = scan_id3v2_header(bytes);
    let header = header_option?;
    let version_str = match header.version {
        ID3v2Version::Id3v2_00 => "-00",
        ID3v2Version::Id3v2_3_0 => ".3.0",
        ID3v2Version::Id3v2_4_0 => ".4.0",
    };
    print!("tag: ID3v2{}", version_str);
    let tag = &bytes[HEADER_LENGTH..header.size + HEADER_LENGTH];
    match header.version {
        ID3v2Version::Id3v2_00 => parse_00(tag, &header),
        ID3v2Version::Id3v2_3_0 => parse_3_0(bytes),
        ID3v2Version::Id3v2_4_0 => parse_4_0(bytes),
    }
    return Some(());
}

fn scan_id3v2_header(bytes: &Vec<u8>) -> Option<ID3v2Header> {
    if bytes.len() < HEADER_LENGTH {
        eprintln!("Too short buffer!");
        return None;
    }
    let header = read_string_from_byte_array(bytes, 0, 3).unwrap_or("");
    if header != TAG {
        eprintln!("No Id3v2 tag!");
        return None;
    }
    let major_version = bytes[MAJOR_VERSION_OFFSET];
    let minor_version = bytes[MINOR_VERSION_OFFSET];
    if major_version != 2 && major_version != 3 && major_version != 4 && minor_version != 0 {
        eprintln!("Unsupported version 2.{}.{}", major_version, minor_version);
        return None;
    }

    let version = match major_version {
        2 => ID3v2Version::Id3v2_00,
        3 => ID3v2Version::Id3v2_3_0,
        4 => ID3v2Version::Id3v2_4_0,
        _ => unreachable!()
    };

    let flags = bytes[FLAGS_OFFSET];
    if flags | 0b00001111 != 0 {
        eprintln!("Invalid flags!");
        return None;
    }


    let tag_size = read_tag_size(&bytes);
    if tag_size.is_err() { return None; }

    return Ok(ID3v2Header {
        version,
        flags,
        size: tag_size.unwrap(),
    });
}

fn read_tag_size(bytes: &Vec<u8>) -> Result<u32, ()> {
    let end = DATA_LENGTH_OFFSET + 4;
    let x = &[
        bytes[DATA_LENGTH_OFFSET + 0],
        bytes[DATA_LENGTH_OFFSET + 1],
        bytes[DATA_LENGTH_OFFSET + 2],
        bytes[DATA_LENGTH_OFFSET + 3],
    ];
    un_synchsafe(&x)
}

fn un_synchsafe(array: &[u8; 4]) -> Result<u32, ()> {
    if array[0] & 0b10000000 != 0 || array[1] & 0b10000000 != 0 || array[2] & 0b10000000 != 0 || array[3] & 0b10000000 != 0 {
        Err(())
    }
    Ok(((array[0] as u32) << 21) +
        ((array[1] as u32) << 14) +
        ((array[2] as u32) << 7) +
        ((array[3] as u32) << 0))
}


#[cfg(test)]
mod tests {
    use id3::id3v2::un_synchsafe;

    #[test]
    fn synchsafe_works() {
        let vec = [0b01111111, 0b01111111, 0b01111111, 0b01111111];
        let i = un_synchsafe(&vec);
        assert_eq!(i, 268435455);
    }

    #[test]
    fn synchsafe_works_2() {
        let vec = [0, 0, 0b00000001, 0b01111111];
        let i = un_synchsafe(&vec);
        assert_eq!(i, 0xFF);
    }

    #[test]
    fn synchsafe_works_3() {
        let vec = [0, 0, 0x02, 0x01];
        let i = un_synchsafe(&vec);
        assert_eq!(i, 257);
    }
}