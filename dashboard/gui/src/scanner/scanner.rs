use crate::scanner::mediatype::{ScanType, IGNORE_EXT, SUPPORTED_EXT};
use crate::scanner::messenger::Messenger;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

use super::messenger;

const BUF_SIZE: usize = 256;
const SCRIPT_NAME: &str = "./duplicates.log";
const EMPTY_STR: &str = "";

pub fn scan(path: &Path, scan_type: ScanType, messenger: Messenger) {
    if !path.is_dir() {
        messenger.push_errlog(format!("{:?} must be a directory", path));
        return;
    }

    // 1. Walk recursivly down from the root_path and group files by size/type
    let mut metas = walk_dir(&path, &scan_type, &messenger);

    // 2. Calculate file checksum from the first BUF_SIZE bytes from
    match scan_type {
        ScanType::BINARY => calc_checksum(&mut metas, &messenger),
        _ => (),
    };

    // 3. Compare complete files if size/type and checksum are equal and build duplicates list
    let duplicates = check_for_duplicates(&scan_type, &metas, &messenger);

    // 4. Print the duplicates to stdout
    match create_bash_script(&duplicates, &messenger) {
        Ok(dups_written) => messenger.info(format!(
            "{} duplicates written to file {}",
            dups_written, SCRIPT_NAME
        )),
        Err(e) => messenger.push_errlog(format!(
            "Could not write file {} due to error {}",
            SCRIPT_NAME, e
        )),
    }
}

/// Scan recursively the file system from the given 'root_path'.
///
/// This creates a HashMap that has a key consisting of length:extension, and holds a list
/// of DirEntry entries for each file.
fn walk_dir(
    root_path: &Path,
    scan_type: &ScanType,
    messenger: &Messenger,
) -> HashMap<String, Vec<FileInfo>> {
    let mut number_of_files: usize = 0;
    let mut fileinfo_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    messenger.info(String::from("Scanning..."));

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir() && !ignore_extension(e.path()))
    {
        if messenger.is_stopped() {
            break;
        }
        match entry.metadata() {
            Ok(metadata) => {
                let file_info = FileInfo::new(entry.clone());
                if !valid_extension(entry.path()) {
                    messenger.push_errlog(file_info.path());
                    continue;
                }

                let key = match scan_type {
                    ScanType::BINARY => file_info.get_key(metadata.len()),
                    ScanType::METADATA => String::new(), // file_info.lofty().unwrap_or("".to_string()),
                };
                number_of_files += 1;
                messenger.push_stdlog(file_info.path());
                messenger.cntmax(number_of_files);
                let entries = fileinfo_map.entry(key).or_insert(Vec::new());
                entries.push(file_info);
            }
            Err(e) => {
                messenger.push_errlog(format!("Error={}", e));
            }
        };
    }

    // Retain only duplicate elements
    fileinfo_map.retain(|_, v| v.len() > 1);
    fileinfo_map
}

/// Calculate a checksums for all files found with have the same size
/// Only if 2 or more files have the same length, the checksum will be calculated.
fn calc_checksum(map: &mut HashMap<String, Vec<FileInfo>>, messenger: &Messenger) {
    let mut count = 0;
    let len = map.len();

    for item in map.values_mut() {
        if messenger.is_stopped() {
            break;
        }

        count += 1;
        messenger.info(String::from("Calculate Checksums"));
        messenger.cntmax(len);
        messenger.cntcur(count);

        if item.len() > 1 {
            for fi in item.into_iter() {
                fi.checksum = get_header_checksum(&fi.path());
            }
        }
    }
}

fn check_for_duplicates(
    scan_type: &ScanType,
    metas: &HashMap<String, Vec<FileInfo>>,
    messenger: &Messenger,
) -> Vec<String> {
    let mut count = 0;
    let len = metas.len();
    let mut duplicates: Vec<String> = Vec::new();

    messenger.info(String::from("Check for duplicates"));
    messenger.cntmax(len);
    messenger.cntcur(count);

    for (key, file_infos) in metas.iter() {
        if messenger.is_stopped() {
            break;
        }

        count += 1;
        messenger.cntcur(count);
        if !key.is_empty() {
            let dups = find_duplicates(&scan_type, &file_infos);
            for dup in dups {
                duplicates.push(dup.clone());
                messenger.push_reslog(dup);
            }
        }
    }
    duplicates
}

fn find_duplicates(scan_type: &ScanType, file_infos: &Vec<FileInfo>) -> HashSet<String> {
    let mut duplicates: HashSet<String> = HashSet::new();

    for i in 0..file_infos.len() - 1 {
        let file_info1 = &file_infos[i];
        for j in i + 1..file_infos.len() {
            let file_info2 = &file_infos[j];
            let mut insert = false;

            match scan_type {
                ScanType::BINARY => {
                    // compare files with identical len
                    if file_info1.checksum - file_info2.checksum == 0 {
                        insert = get_file_checksum(&file_info1.path())
                            == get_file_checksum(&file_info2.path());
                    }
                }
                ScanType::METADATA => insert = true,
            };

            if insert {
                duplicates.insert(file_info1.path());
                duplicates.insert(file_info2.path());
            }
        }
    }
    duplicates
}

/// Reads the first BUF_SIZE bytes from a file and creates a isize checksum.
///
/// Returns the isize checksum
fn get_header_checksum(path: &str) -> isize {
    let f = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}, path={}", e, path);
            return 0;
        }
    };

    let mut reader = BufReader::new(f);
    let mut buffer = [0u8; BUF_SIZE];

    let bytes_read = match reader.read(&mut buffer) {
        Ok(bytes_read) => bytes_read,
        Err(e) => {
            eprintln!("Error: {}, path={}", e, path);
            return 0;
        }
    };

    let mut checksum: isize = 0;
    for i in 0..bytes_read {
        checksum += buffer[i] as isize;
    }
    checksum
}

/// Reads all bytes from a file calculate a isize checksum.
///
/// Returns the isize checksum
fn get_file_checksum(path: &str) -> isize {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}, path={}", e, path);
            return 0;
        }
    };

    //let mut contents: Vec<u8> = Vec::new();
    let mut contents: Vec<u8> = Vec::new();
    if let Err(e) = file.read_to_end(&mut contents) {
        eprintln!("Could not read file! Error: {}, path={}", e, path);
        return 0;
    } else {
        let mut checksum: isize = 0;
        for b in contents.into_iter() {
            checksum += b as isize;
        }
        checksum
    }
}

fn create_bash_script(
    duplicates: &Vec<String>,
    messenger: &Messenger,
) -> Result<usize, std::io::Error> {
    if !messenger.is_stopped() {
        let mut f = File::create(SCRIPT_NAME)?;
        for path in duplicates.iter() {
            f.write_all(path.as_bytes())?;
            f.write_all(&[b'\n'])?;
        }
        Ok(duplicates.len())
    } else {
        Ok(0)
    }
}

pub fn get_extension(path: &str) -> String {
    let extension = match path.rfind('.') {
        Some(idx) => format!("{}", &path[idx..].to_uppercase()),
        None => format!("{}", ""),
    };
    extension
}

pub fn ignore_extension(path: &Path) -> bool {
    let full_path = path.to_str().unwrap_or("");
    let ignore = match full_path.rfind('.') {
        Some(idx) => IGNORE_EXT.contains(&full_path[idx..].to_uppercase()),
        None => false,
    };
    ignore
}

pub fn valid_extension(path: &Path) -> bool {
    let full_path = path.to_str().unwrap_or("");
    let valid = match full_path.rfind('.') {
        Some(idx) => SUPPORTED_EXT.contains(&full_path[idx..].to_uppercase()),
        None => false,
    };
    valid
}

#[derive(Debug)]
struct FileInfo {
    dir_entry: DirEntry,
    checksum: isize,
}
impl FileInfo {
    pub fn new(dir_entry: DirEntry) -> FileInfo {
        FileInfo {
            dir_entry: dir_entry,
            checksum: 0,
        }
    }

    /// Returns the String representation of file's path
    pub fn path(&self) -> String {
        match self.dir_entry.path().to_str() {
            Some(s) => return String::from(s),
            None => return String::from(""),
        };
    }

    /// Returns a String as key for the hashmap.
    ///
    /// Key is build by combining the file length with the extension.
    /// If no extension is found, then only the file length will be used as key
    pub fn get_key(&self, length: u64) -> String {
        let path = self.path();
        let key = match path.rfind('.') {
            Some(idx) => format!("{}{}", length, &path[idx..].to_uppercase()),
            None => format!("{}", length),
        };
        key
    }

    // Read audio metadata
}
