use crate::scanner::mediatype::{MediaGroup, ScanType};
use crate::scanner::messenger::Messenger;
use crate::components::basic::file_utils::*;
use crate::components::basic::lofty_utils::*;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Write},
    path::Path,
};
//use std::fmt::Pointer;
//use std::time::Duration;
use walkdir::{DirEntry, WalkDir};
use crate::components::basic::string_utils::normalize_option;


const SCRIPT_NAME: &str = "./duplicates.log";

pub fn scan(path: &Path, scan_type: ScanType, media_groups: Vec<MediaGroup>, messenger: Messenger) {
    if !path.is_dir() {
        messenger.push_errlog(format!("{:?} must be a directory", path));
        return;
    }

    // 1. Walk recursive down from the root_path and group files by size/type
    let mut metas = walk_dir(&path, &scan_type, &media_groups, &messenger);

    // 2. Calculate file checksum from the first BUF_SIZE bytes from
    match scan_type {
        ScanType::BINARY => calc_checksum(&mut metas, &messenger),
        _ => (),
    };

    // 3. Compare complete files if size/type and checksum are equal and build duplicates list
    let duplicates = check_for_duplicates(&scan_type, &metas, &messenger);

    // 4. Print the duplicates to stdout
    match create_bash_script(&duplicates, &messenger) {
        Ok(duplicates_written) => {
            if duplicates_written > 0 {
                messenger.set_info(format!("{} duplicates written to file {}", duplicates_written, SCRIPT_NAME))
            } else {
                messenger.set_info("".to_string());
            }
        }
        Err(e) => messenger.push_errlog(format!("Could not write file {} due to error {}", SCRIPT_NAME, e)),
    }
}

/// Scan recursively the file system from the given 'root_path'.
///
/// This creates a HashMap that has a key consisting of length:extension, and holds a list
/// of DirEntry entries for each file.
fn walk_dir(
    root_path: &Path,
    scan_type: &ScanType,
    media_groups: &Vec<MediaGroup>,
    messenger: &Messenger,
) -> HashMap<String, Vec<FileInfo>> {
    let mut fileinfo_map: HashMap<String, Vec<FileInfo>> = HashMap::new();

    messenger.set_info("Scanning...".to_owned());

    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.path().is_dir())
    {
        if messenger.is_stopped() || messenger.is_interrupted() {
            break;
        }

        match entry.metadata() {
            Ok(metadata) => {
                let file_info = FileInfo::new(entry.clone());
                let extension = get_extension(file_info.path_to_str());
                // If unknown extension then log in error
                if !media_groups.iter().any(|mg| mg.is_known_extension(&extension)) {
                    messenger.push_errlog(format!("Unknown extension {}", file_info.path_to_str()));
                    continue;
                }
                // Known extension and selected?
                if !media_groups.iter().any(|mg| mg.is_selected(&extension)) {
                    continue;
                }

                messenger.push_stdlog(format!("{}", file_info.path_to_str()));

                let key: String;
                match scan_type {
                    ScanType::BINARY => {
                        key = file_info.get_key(metadata.len());
                    } // binary
                    ScanType::METADATA => {
                        let map = get_audio_tags(&file_info.path());
                        let duration: usize = map.get("Duration").unwrap_or(&"0".to_string()).parse().unwrap();
                        let track_title = normalize_option(map.get("TrackTitle"));

                        if duration == 0 || track_title.is_empty() {
                            messenger.push_errlog(format!("No Duration '{}' or TrackTitle '{}' for {}", duration, track_title, file_info.path_to_str()));
                            continue;
                        }
                        key = format!("{}{}", duration, track_title);
                    } // metadata
                } // match ScanType

                // Add key to list
                let entries = fileinfo_map.entry(key).or_insert(Vec::new());
                entries.push(file_info);
            } // (Ok) metadata
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
            messenger.set_progress(0, 0, "");
            break;
        }

        count += 1;
        messenger.set_progress(len, count, "Calculate Checksums...");

        if item.len() > 1 {
            for fi in item.into_iter() {
                fi.checksum = get_header_checksum(&fi.path()).unwrap_or(0);
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

    for (key, file_infos) in metas.iter() {
        if messenger.is_stopped() {
            break;
        }

        count += 1;
        messenger.set_progress(len, count, "Check for duplicates...");
        if !key.is_empty() {
            for dup in find_duplicates(&scan_type, &file_infos, &messenger) {
                duplicates.push(dup.clone());
                messenger.push_reslog(dup);
            }
        }
    }
    duplicates
}

fn find_duplicates(scan_type: &ScanType, file_infos: &Vec<FileInfo>, messenger: &Messenger) -> HashSet<String> {
    let mut duplicates: HashSet<String> = HashSet::new();

    for i in 0..file_infos.len() - 1 {
        let file_info1 = &file_infos[i];
        for j in i + 1..file_infos.len() {
            let file_info2 = &file_infos[j];
            let mut insert = false;

            match scan_type {
                ScanType::BINARY => {
                    // compare files with identical headers
                    if file_info1.checksum == 0 || file_info2.checksum == 0 {
                        messenger.push_errlog(format!("No checksum for file {}", file_info1.path_to_str()));
                        continue;
                    }
                    if file_info1.checksum - file_info2.checksum == 0 {
                        let checksum1 = compute_file_checksum(file_info1.path()).unwrap();
                        let checksum2 = compute_file_checksum(file_info2.path()).unwrap();
                        insert = checksum1 == checksum2;
                    }
                }
                ScanType::METADATA => {
                    let key1 = get_audio_key(&get_audio_tags(file_info1.path()));
                    let key2 = get_audio_key(&get_audio_tags(file_info2.path()));
                    insert = key1 == key2;
                }
            };

            if insert {
                duplicates.insert(file_info1.path_to_str().to_string());
                duplicates.insert(file_info2.path_to_str().to_string());
            }

            if messenger.is_stopped() {
                break;
            }
        }
    }
    duplicates
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

#[derive(Debug)]
struct FileInfo {
    dir_entry: DirEntry,
    checksum: isize,
}

impl FileInfo {
    pub fn new(dir_entry: DirEntry) -> FileInfo {
        FileInfo {
            dir_entry,
            checksum: 0,
        }
    }

    pub fn path_to_str(&self) -> &str {
        self.dir_entry.path().to_str().unwrap()
    }

    pub fn path(&self) -> &Path {
        self.dir_entry.path()
    }

    /// Returns a String as key for the hashmap.
    ///
    /// Key is build by combining the file length with the extension.
    /// If no extension is found, then only the file length will be used as key
    pub fn get_key(&self, length: u64) -> String {
        let path = self.path_to_str();
        let key = match path.rfind('.') {
            Some(idx) => format!("{}{}", length, &path[idx..].to_uppercase()),
            None => format!("{}", length),
        };
        key
    }
}
