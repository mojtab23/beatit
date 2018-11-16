use id3::TAG;
use id3::read_string_from_byte_array;
use std::str;
use std::u8;

pub struct Id3v1 {
    pub track: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub year: String,
    pub comment: String,
}

const TAG_LENGTH: usize = 128;
const TITLE_OFFSET: usize = 3;
const TITLE_LENGTH: usize = 30;
const ARTIST_OFFSET: usize = 33;
const ARTIST_LENGTH: usize = 30;
const ALBUM_OFFSET: usize = 63;
const ALBUM_LENGTH: usize = 30;
const YEAR_OFFSET: usize = 93;
const YEAR_LENGTH: usize = 4;
const COMMENT_OFFSET: usize = 97;
const COMMENT_LENGTH_V1_0: usize = 30;
const COMMENT_LENGTH_V1_1: usize = 28;
const TRACK_MARKER_OFFSET: usize = 125;
const TRACK_OFFSET: usize = 126;
const GENRE_OFFSET: usize = 127;

pub fn parse_id3v1(file_bytes: &Vec<u8>) -> Result<Id3v1, ()> {
    let v1_bytes = read_id3v1_tag_bytes(&file_bytes).expect("");
    //            read tag name
    let tag;
    let slice = &v1_bytes[..3];
    let result = str::from_utf8(slice);
    if result.is_err() {
        println!("No Id3v1 tag.");
        return Err(());
    } else {
        tag = result.unwrap()
    }
    if !tag.eq(TAG) {
        println!("No Id3v1 tag.");
        return Err(());
    }
    //            read title
    let title = read_string_from_byte_array(v1_bytes, TITLE_OFFSET, TITLE_LENGTH).unwrap_or("");
    println!("Title:{}", title);
    let artist = read_string_from_byte_array(v1_bytes, ARTIST_OFFSET, ARTIST_LENGTH).unwrap_or("");
    println!("Artist:{}", artist);
    let album = read_string_from_byte_array(v1_bytes, ALBUM_OFFSET, ALBUM_LENGTH).unwrap_or("");
    println!("Album:{}", album);
    let year = read_string_from_byte_array(v1_bytes, YEAR_OFFSET, YEAR_LENGTH).unwrap_or("");
    println!("Year:{}", year);

    let genre = v1_bytes[GENRE_OFFSET] & 0xFF;
    if genre == 0xFF {
//                genre = -1;
//                has no genre!
    }

    let comment;
    let track;
    if v1_bytes[TRACK_MARKER_OFFSET] != 0 {
        comment = read_string_from_byte_array(v1_bytes, COMMENT_OFFSET, COMMENT_LENGTH_V1_0).unwrap_or("");
        track = 0;
    } else {
        comment = read_string_from_byte_array(v1_bytes, COMMENT_OFFSET, COMMENT_LENGTH_V1_1).unwrap_or("");
        let track_number = v1_bytes[TRACK_OFFSET];
        if track_number == 0 {
            track = 0;
        } else {
            track = track_number;
        }
    }
    println!("Comment:{}", comment);
    println!("Track:{}", track);
    return Ok(Id3v1 {
        track: track.to_string(),
        artist: artist.to_string(),
        title: title.to_string(),
        album: album.to_string(),
        year: year.to_string(),
        comment: comment.to_string(),
    });
}

//fn read_string_from_byte_array(byte_array: &[u8], start_index: usize, length: usize) -> &str {
//    let end_index = start_index + length;
//    let slice = &byte_array[start_index..end_index];
//    let str = format!("There is no valid string in [{}..{}].", start_index, end_index);
//    str::from_utf8(slice).expect(str.as_str())
//}

fn read_id3v1_tag_bytes(file_bytes: &Vec<u8>) -> Result<&[u8], ()> {
    let file_length = file_bytes.len();
    if file_length < TAG_LENGTH {
        eprintln!("File is too small!");
        return Err(());
    }
    let id3v1_bytes = &file_bytes[file_length - TAG_LENGTH..file_length];
    println!("ld3v1 length is {}.", id3v1_bytes.len());
    return Ok(id3v1_bytes);
}
