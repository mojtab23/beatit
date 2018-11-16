extern crate beatit;

use beatit::id3::read_tags;
use std::fs;
use std::process;


fn main() {
//    let file_name = "resources/v1tag.mp3";
    let file_name = "resources/v1andv23tags.mp3";
//    let file_name = "resources/notags.mp3";
    let file_bytes = read_file(file_name);
//    let result = parse_id3v1(&file_bytes);
    read_tags(&file_bytes);

//    if result.is_ok() {
//        println!("{}", result.unwrap().title);
//    } else {
//        println!("File has no tag in it.");
//    }
}

fn read_file(file_name: &str) -> Vec<u8> {
    let result = fs::read(file_name);
    match result {
        Ok(data) => {
            println!("file length is {}.", data.len());
            data
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    }
}


