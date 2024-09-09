use crate::scanner::mediatype::{MediaGroup, ScanType};
use crate::scanner::messenger::Messenger;
use crate::components::basic::file_utils::*;
use crate::components::basic::lofty_utils::*;

use std::{
    collections::HashMap,
    fs::File,
    io::{Write},
    path::Path,
};
use walkdir::{DirEntry, WalkDir};

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
    match create_bash_script(&duplicates) {
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

        if entry.metadata().is_err() {
            messenger.push_errlog(format!("Error={:?}", entry.metadata().err()));
            continue;
        }

        let metadata = entry.metadata().ok().unwrap();
        let file_info = FileInfo::new(entry.clone());
        let extension = get_extension(file_info.path_to_str());

        // If unknown extension then log in error
        if !media_groups.iter().any(|mg| (mg.is_known_extension(&extension) && mg.is_selected(&extension))) {
            messenger.push_errlog(format!("Extension {} ignored: {}", &extension, file_info.path_to_str()));
            continue;
        }

        let key: String;
        match scan_type {
            ScanType::BINARY => {
                key = file_info.get_key(metadata.len());
            } // binary
            ScanType::METADATA => {
                key = match get_short_audio_key(&file_info.path()) {
                    Ok(key) => key,
                    Err(e) => {
                        messenger.push_errlog(format!("{:?} : file: {:?}", e.to_string(), file_info.path()));
                        continue;
                    }
                }
            } // metadata
        } // match ScanType

        // Add key to list
        messenger.push_stdlog(format!("{} : {}", key, file_info.path_to_str()));
        let entries = fileinfo_map.entry(key).or_insert(Vec::new());
        entries.push(file_info);
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
                match get_header_checksum(&fi.path()) {
                    Ok(checksum) => fi.checksum = checksum,
                    Err(e) => messenger.push_errlog(format!("Error getting checksum for file {:?} : {:?}", &fi.path(), e.to_string()))
                }
            }
        }
    }
}

fn check_for_duplicates(
    scan_type: &ScanType,
    metas: &HashMap<String, Vec<FileInfo>>,
    messenger: &Messenger,
) -> Vec<HashMap<String, String>> {
    let mut count = 0;
    let len = metas.len();
    let mut duplicates: Vec<HashMap<String, String>> = Vec::new();

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

fn find_duplicates(scan_type: &ScanType, file_infos: &Vec<FileInfo>, messenger: &Messenger) -> Vec<HashMap<String, String>> {
    let mut duplicates: Vec<HashMap<String, String>> = Vec::new();

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
                    // Botch checksums are equal
                    if file_info1.checksum - file_info2.checksum == 0 {
                        let get_checksum = |path: &Path, messenger: &Messenger| -> String {
                            match compute_file_checksum(path) {
                                Ok(checksum) => checksum,
                                Err(e) => {
                                    messenger.push_errlog(format!("Error for file {:?} : {:?}", path, e.to_string()));
                                    String::new()
                                }
                            }
                        };
                        insert = get_checksum(file_info1.path(), &messenger) == get_checksum(file_info2.path(), &messenger);
                    }
                }
                ScanType::METADATA => {
                    let unwrap = |path: &Path| -> String {
                        match get_audio_key(path) {
                            Ok(key) => key,
                            Err(e) => {
                                messenger.push_errlog(format!("Error Could not get Key file {:?} : {:?}", file_info1.path(), e.to_string()));
                                String::new()
                            }
                        }
                    };

                    let key1 = unwrap(file_info1.path());
                    let key2 = unwrap(file_info2.path());
                    insert = key1 == key2;
                }
            };

            if insert {
                duplicates.push(get_audio_tags(file_info1.path()).unwrap());
                duplicates.push(get_audio_tags(file_info2.path()).unwrap());
            }

            if messenger.is_stopped() {
                break;
            }
        }
    }
    duplicates
}

fn create_bash_script(
    duplicates: &Vec<HashMap<String, String>>,
) -> Result<usize, std::io::Error> {
    let mut f = File::create(SCRIPT_NAME)?;
    for item in duplicates {
        writeln!(f, "{:?}", item.get("PATH").unwrap())?;

        //TODO: Where is item.keys().sorted() function?
        let mut vkeys = item.keys().cloned().collect::<Vec<_>>();
        vkeys.sort();
        for key in vkeys.iter() {
            let val = item.get(key).unwrap();
            writeln!(f, "\t\t{}: {}", key, val)?;
        }
    }
    Ok(duplicates.len())
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
