use std::{
    path::Path,
    fs::File,
    io::Read,
    fmt::Pointer,
};
use anyhow::Result;


pub fn get_extension(path: &str) -> String {
    let extension = match path.rfind('.') {
        Some(idx) => (&path[idx..].to_uppercase()).to_owned(),
        None => String::new(),
    };
    extension
}

pub fn compute_file_checksum(file: &Path) -> Result<String> {
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