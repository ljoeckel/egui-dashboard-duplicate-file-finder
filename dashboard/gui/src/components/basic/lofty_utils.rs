use std::path::Path;

use std::collections::HashMap;
use lofty::file::AudioFile;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use crate::components::basic::string_utils::*;
use anyhow::{Result, Error, anyhow};

fn unwrap(s: Option<&String>) -> String {
    s.unwrap_or(&"".to_string()).to_string()
}

pub fn get_audio_tags(file: &Path) -> Result<HashMap<String, String>, Error> {
    let mut map: HashMap<String, String> = HashMap::new();
    // Insert the file Path as PATH entry
    map.insert("PATH".to_string(), file.to_str().unwrap().to_string());

    let probe = Probe::open(file)?;
    let tagged_file = probe.read()?;

    tagged_file.primary_tag();
    // Extract the Tag from the file if any
    if let Some(tag) = tagged_file.primary_tag() {
        for item in tag.items().into_iter() {
            let value = item.value().text().unwrap_or("");
            let key = format!("{:?}", item.key());
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
    Ok(map)
}

pub fn get_short_audio_key(file: &Path) -> Result<String, Error> {
    match get_audio_tags(file) {
        Ok(map) => {
            // Get Track
            let track = normalize_option(map.get("TrackTitle"));
            if track.is_empty() {
                return Err(anyhow!("TrackTitle is empty"));
            }
            // Get Duration
            let duration = unwrap(map.get("Duration"));
            if duration.is_empty() {
                return Err(anyhow!("Duration=0"));
            }

            Ok(format!("{}{}", duration, track))
        },
        Err(e) => Err(e)
    }
}

pub fn get_audio_key(file: &Path) -> Result<String, Error> {
    match get_audio_tags(file) {
        Ok(map) => {
            // Get Artist
            let mut artist = unwrap(map.get("AlbumArtist"));
            if artist.is_empty() {
                artist = unwrap(map.get("TrackArtist"));
            }
            // Get Album
            let mut album = unwrap(map.get("AlbumTitle"));
            if album.is_empty() {
                album = unwrap(map.get("OriginalAlbumTitle"));
            }
            // Get Track
            let track = normalize_option(map.get("TrackTitle"));
            // Get Duration
            let mut duration = unwrap(map.get("Duration"));
            if duration.is_empty() {
                duration = "0".to_string();
            }

            Ok(format!("{}{}{}{}", duration, artist, album, track))
        },
        Err(e) => Err(e)
    }
}
