use id3::id3v1::parse_id3v1;
use id3::id3v2::parse_id3v2;
use std::str;
use std::str::Utf8Error;

const TAG: &str = "TAG";
const DEFAULT_BUFFER_LENGTH: usize = 65536;
const MINIMUM_BUFFER_LENGTH: usize = 40;


pub mod id3v1;
pub mod id3v2;

pub fn read_tags(bytes: &Vec<u8>) {
    let id3v1_result = parse_id3v1(bytes);
    parse_id3v2(bytes);
}


fn read_string_from_byte_array(byte_array: &[u8], start_index: usize, length: usize) -> Result<&str,Utf8Error >{
    let end_index = start_index + length;
    let slice = &byte_array[start_index..end_index];
    let str = format!("There is no valid string in [{}..{}].", start_index, end_index);
    str::from_utf8(slice)
}