use std::{
    path::Path,
    fs::File,
    io::Read,
};
use std::io::{BufReader, Error};

const BUF_SIZE: usize = 256;

/// Reads the first BUF_SIZE bytes from a file and creates an isize checksum.
///
/// Returns the isize checksum
pub fn get_header_checksum(path: &Path) -> Result<isize, Error> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = [0u8; BUF_SIZE];

    let bytes_read = reader.read(&mut buffer)?;
    let mut checksum: isize = 0;
    for i in 0..bytes_read {
        checksum += buffer[i] as isize * i as isize;
    }
    Ok(checksum)
}

pub fn get_extension(path: &str) -> String {
    let extension = match path.rfind('.') {
        Some(idx) => (&path[idx..].to_uppercase()).to_owned(),
        None => String::new(),
    };
    extension
}

pub fn compute_file_checksum(file: &Path) -> Result<String, Error> {
    let mut file = File::open(file)?;
    let mut buffer = [0; 4096];
    let mut sum: usize = 0; // summarize all bytes into
    let mut block = 0;
    loop {
        let bytes_read = file.read(&mut buffer)?;
        block += 1;
        if bytes_read == 0 {
            break;
        }
        for i in 0..bytes_read {
            sum += *&buffer[i] as usize * block;
        }
    }
    Ok(sum.to_string())
}