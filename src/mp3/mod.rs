pub fn read_mp3_frames(bytes: &[u8], position: usize) {
    read_mp3_frame(bytes,position);

}


fn read_mp3_frame(bytes: &[u8], position: usize){
    read_frame_header(bytes,position)
}

fn read_frame_header(bytes: &[u8], position: usize){
    bytes[position]

}
