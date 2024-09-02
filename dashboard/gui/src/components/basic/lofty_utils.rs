use std::path::Path;

use std::collections::HashMap;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use crate::components::basic::string_utils::*;

pub fn get_audio_tags(file: &Path) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    match Probe::open(file) {
        Ok(probe) => {
            match probe.read() {
                Ok(tagged_file) => {
                    tagged_file.primary_tag();
                    // Extract the Tag from the file if any
                    if let Some(tag) = tagged_file.primary_tag() {
                        for item in tag.items().into_iter() {
                            let value = item.value().text().unwrap_or("");
                            let key = format!("{:?}", item.key()); //ItemKey::AlbumArtistSortOrder,,
                            map.insert(key, value.to_string());
                        }

                        let properties = tagged_file.properties();
                        map.insert("Duration".to_string(), properties.duration().as_secs().to_string());
                        map.insert("SampleRate".to_string(), properties.sample_rate().unwrap_or(0).to_string());
                        map.insert("Channels".to_string(), properties.channels().unwrap_or(0).to_string());
                        map.insert("ChannelMask".to_string(), format!("{:?}", properties.channel_mask().unwrap_or_default()));
                        map.insert("AudioBitrate".to_string(), properties.audio_bitrate().unwrap_or(0).to_string());
                        map.insert("OverallBitrate".to_string(), properties.overall_bitrate().unwrap_or(0).to_string());
                        map.insert("BitDepth".to_string(), properties.bit_depth().unwrap_or(0).to_string());
                    }
                }
                Err(e) => println!("error {}", e)
            }
        }
        Err(e) => println!("{:?}, open file {:?}", e, file)
    }
    map
}

pub fn _audio_key(map: &HashMap<String, String>) -> String {
    let key = format!("{:?}{:?}{:?}{:?}",
                      map.get("Duration"),
                      map.get("AlbumArtist"),
                      map.get("AlbumTitle"),
                      map.get("TrackTitle"),
    );
    key.to_lowercase()
}

pub fn audio_key(file: &Path) -> String {
    let map = get_audio_tags(file);
    return _audio_key(&map);
}

pub fn get_audio_key(map: &HashMap<String, String>) -> String {
    let unwrp = |s: Option<&String> | -> String {
        s.unwrap_or(&"".to_string()).to_string()
    };

    // Get Artist
    let mut artist = unwrp(map.get("AlbumArtist"));
    if artist.is_empty() {
        artist = unwrp(map.get("TrackArtist"));
    }
    // Get Album
    let mut album = unwrp(map.get("AlbumTitle"));
    if album.is_empty() {
        album = unwrp(map.get("OriginalAlbumTitle"));
    }
    // Get Track
    let track = normalize_option(map.get("TrackTitle"));
    // Get Duration
    let mut duration = unwrp(map.get("Duration"));
    if duration.is_empty() {
        duration = "0".to_string();
    }

    return format!("{}{}{}{}", duration, artist, album, track);
}

