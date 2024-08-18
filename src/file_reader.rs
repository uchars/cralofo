use std::{
    fs::File,
    io::{self, BufRead, Seek},
};

use log::info;

use crate::models::logs::FileRead;

pub fn read_lines_starting_from_byte(
    file_path: &str,
    start_byte: u64,
    buf_size: usize,
) -> Option<FileRead> {
    info!("starting read of {} from byte {}", file_path, start_byte);
    let mut file = File::open(file_path).ok()?;
    file.seek(io::SeekFrom::Start(start_byte)).ok()?;

    let mut reader = io::BufReader::new(file);
    let mut lines = Vec::new();
    let mut curr_pos = start_byte;
    let mut total_bytes = 0;

    // loop until EOF or exceeding buffer size.
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).ok()?;
        if bytes_read == 0 {
            // EOF
            break;
        }

        if line.ends_with('\n') {
            line.pop();
        }

        if total_bytes + bytes_read > buf_size {
            // adding line would exceed buffer size.
            break;
        }

        total_bytes += bytes_read;
        curr_pos += bytes_read as u64;
        lines.push(line);
    }

    info!("({}) {} lines read", file_path, lines.len());

    Some(FileRead {
        lines,
        new_pos: curr_pos,
    })
}
