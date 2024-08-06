use std::collections::HashMap;

#[derive(Debug)]
pub enum ScanType {
    BINARY,
    METADATA,
}
#[derive(Debug)]
pub enum Control {
    STOP,
    INFO,
}

#[derive(Debug)]
pub struct MediaType {
    extension: String,
    description: String,
    selected: bool,
}

impl MediaType {
    pub fn new(extension: &str, description: &str) -> MediaType {
        MediaType {
            extension: String::from(extension),
            description: String::from(description),
            selected: true,
        }
    }
}

#[derive(Debug)]
pub struct MediaMap {
    pub media_group: String,
    pub map: HashMap<String, Vec<MediaType>>,
    pub selected: bool,
}
impl MediaMap {
    fn new(media_group: &str) -> MediaMap {
        MediaMap {
            media_group: String::from(media_group),
            map: HashMap::new(),
            selected: true,
        }
    }

    pub fn load_maps() -> Vec<MediaMap> {
        let mut maps: Vec<MediaMap> = Vec::new();

        let mut media_map = MediaMap::new("audio");
        media_map.map = audio_types();
        maps.push(media_map);

        let mut media_map = MediaMap::new("video");
        media_map.map = video_types();
        maps.push(media_map);

        maps
    }
}

fn audio_types() -> HashMap<String, Vec<MediaType>> {
    let key = "audio";
    let mut map: HashMap<String, Vec<MediaType>> = HashMap::new();

    add_type(key, &mut map, ".3gp", "Multimedia container format can contain proprietary formats as AMR, AMR-WB or AMR-WB+, but also some open formats");
    add_type(key, &mut map, ".aa", "A low-bitrate audiobook container format with DRM, containing audio encoded as either MP3 or the ACELP speech codec.");
    add_type(key, &mut map, ".aac", "The Advanced Audio Coding format is based on the MPEG-2 and MPEG-4 standards. AAC files are usually ADTS or ADIF containers.");
    add_type(key, &mut map, ".aax", "An Audiobook format, which is a variable-bitrate (allowing high quality) M4B file encrypted with DRM. MPB contains AAC or ALAC encoded audio in an MPEG-4 container. (More details below.)");
    add_type(key, &mut map, ".act", "ACT is a lossy ADPCM 8 kbit/s compressed audio format recorded by most Chinese MP3 and MP4 players with a recording function, and voice recorders");
    add_type(key, &mut map, ".aiff", "A standard uncompressed CD-quality, audio file format used by Apple. Established 3 years prior to Microsoft's uncompressed version wav.");
    add_type(key, &mut map, ".alac", "An audio coding format developed by Apple Inc. for lossless data compression of digital music.");
    add_type(
        key,
        &mut map,
        ".amr",
        "AMR-NB audio, used primarily for speech.",
    );
    add_type(
        key,
        &mut map,
        ".ape",
        "Monkey's Audio lossless audio compression format.",
    );
    add_type(key, &mut map, ".au", "The standard audio file format used by Sun, Unix and Java. The audio in au files can be PCM or compressed with the μ-law, a-law or G.729 codecs.");
    add_type(
        key,
        &mut map,
        ".awb",
        "AMR-WB audio, used primarily for speech, same as the ITU-T's G.722.2 specification.",
    );
    add_type(key, &mut map, ".dss", "DSS files are an Olympus proprietary format. DSS files use a high compression rate, which reduces the file size and allows files to be copied and transferred quickly.[6] It allows additional data to be held in the file header.");
    add_type(key, &mut map, ".dvf", "A Sony proprietary format for compressed voice files; commonly used by Sony dictation recorders.");
    add_type(key, &mut map, ".flac", "A file format for the Free Lossless Audio Codec, an open-source lossless compression codec.");
    add_type(key, &mut map, ".gsm", "Designed for telephony use in Europe, GSM is used to store telephone voice messages and conversations. With a bitrate of 13kbps, GSM files can compress and encode audio at telephone quality. [7] Note that WAV files can also be encoded with the GSM codec.");
    add_type(key, &mut map, ".iklax", "An iKlax Media proprietary format, the iKlax format is a multi-track digital audio format allowing various actions on musical data, for instance on mixing and volumes arrangements.");
    add_type(key, &mut map, ".ivs", "A proprietary version with DRM developed by 3D Solar UK Ltd for use in music downloaded from their Tronme Music Store and interactive music and video player.");
    add_type(key, &mut map, ".m4a", "An audio-only MPEG-4 file, used by Apple for unprotected music downloaded from their iTunes Music Store. Audio within the m4a file is typically encoded with AAC, although lossless ALAC may also be used.");
    add_type(key, &mut map, ".m4b", "An Audiobook / podcast extension with AAC or ALAC encoded audio in an MPEG-4 container. Both M4A and M4B formats can contain metadata including chapter markers, images, and hyperlinks, but M4B allows 'bookmarks' (remembering the last listening spot), whereas M4A does not.[8]");
    add_type(key, &mut map, ".m4p", "A version of AAC with proprietary DRM developed by Apple for use in music downloaded from their iTunes Music Store and their music streaming service known as Apple Music.");
    add_type(key, &mut map, ".mmf", "A Samsung audio format that is used in ringtones. Developed by Yamaha (SMAF stands for 'Synthetic music Mobile Application Format', and is a multimedia data format invented by the Yamaha Corporation, .mmf file format).");
    add_type(key, &mut map, ".movpkg", "An Apple audio format primarily used for Lossless and Hi-Res audio files through Apple Music. Also used for storing Apple TV videos.");
    add_type(key, &mut map, ".mp3", "MPEG Layer III Audio");
    add_type(key, &mut map, ".mpc", "Musepack or MPC (formerly known as MPEGplus, MPEG+ or MP+) is an open source lossy audio codec, specifically optimized for transparent compression of stereo audio at bitrates of 160–180 kbit/s.");
    add_type(
        key,
        &mut map,
        ".msv",
        "A Sony proprietary format for Memory Stick compressed voice files.",
    );
    add_type(key, &mut map, ".nmf", "NICE Media Player audio file");
    add_type(key, &mut map, ".ogg", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format.");
    add_type(key, &mut map, ".oga", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format.");
    add_type(key, &mut map, ".mogg", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format.");
    add_type(key, &mut map, ".opus", "A lossy audio compression format developed by the Internet Engineering Task Force (IETF) and made especially suitable for interactive real-time applications over the Internet. As an open format standardised through RFC 6716, a reference implementation is provided under the 3-clause BSD license.");
    add_type(key, &mut map, ".ra", "A RealAudio format designed for streaming audio over the Internet. The .ra format allows files to be stored in a self-contained fashion on a computer, with all of the audio data contained inside the file itself.");
    add_type(key, &mut map, ".rm", "A RealAudio format designed for streaming audio over the Internet. The .ra format allows files to be stored in a self-contained fashion on a computer, with all of the audio data contained inside the file itself.");
    add_type(key, &mut map, ".raw", "A raw file can contain audio in any format but is usually used with PCM audio data. It is rarely used except for technical tests.");
    add_type(
        key,
        &mut map,
        ".rf64",
        "One successor to the Wav format, overcoming the 4GiB size limitation.",
    );
    add_type(key, &mut map, ".sln", "Signed Linear PCM format used by Asterisk. Prior to v.10 the standard formats were 16-bit Signed Linear PCM sampled at 8 kHz and at 16 kHz. With v.10 many more sampling rates were added.[9]");
    add_type(
        key,
        &mut map,
        ".tta",
        "The True Audio, real-time lossless audio codec.",
    );
    add_type(key, &mut map, ".voc", "The file format consists of a 26-byte header and a series of subsequent data blocks containing the audio information");
    add_type(key, &mut map, ".vox", "The vox format most commonly uses the Dialogic ADPCM (Adaptive Differential Pulse Code Modulation) codec. Similar to other ADPCM formats, it compresses to 4-bits. Vox format files are similar to wave files except that the vox files contain no information about the file itself so the codec sample rate and number of channels must first be specified in order to play a vox file.");
    add_type(key, &mut map, ".wav", "Standard audio file container format used mainly in Windows PCs. Commonly used for storing uncompressed (PCM), CD-quality sound files, which means that they can be large in size—around 10 MB per minute. Wave files can also contain data encoded with a variety of (lossy) codecs to reduce the file size (for example the GSM or MP3 formats). Wav files use a RIFF structure.");
    add_type(key, &mut map, ".wma", "Windows Media Audio format, created by Microsoft. Designed with DRM abilities for copy protection.");
    add_type(key, &mut map, ".wv", "Format for wavpack files.");
    add_type(
        key,
        &mut map,
        ".webm",
        "Royalty-free format created for HTML video.",
    );
    add_type(key, &mut map, ".8svx", "The IFF-8SVX format for 8-bit sound samples, created by Electronic Arts in 1984 at the birth of the Amiga.");
    add_type(key, &mut map, ".cda", "Format for cda files for Radio.");
    map
}

fn video_types() -> HashMap<String, Vec<MediaType>> {
    let key = "video";
    let mut map: HashMap<String, Vec<MediaType>> = HashMap::new();

    add_type(key, &mut map, ".vob", "Multimedia container format can contain proprietary formats as AMR, AMR-WB or AMR-WB+, but also some open formats");
    map
}

fn add_type(
    key: &str,
    map: &mut HashMap<String, Vec<MediaType>>,
    extension: &str,
    description: &str,
) {
    let mt = MediaType::new(extension, description);
    let entries = map.entry(String::from(key)).or_insert(Vec::new());
    entries.push(mt);
}

pub const IGNORE_EXT: &str =
    ".MF,.GITIGNORE,.RLIB,.RMETA,.BIN,.TIMESTAMP,.IDX,.LOCK,.A,.O,.DS_STORE,._.DS_SSTORE,.M3U,.NFO,.RTF,.SFV,.URL,.WPL,.LOG,.BAK";

pub const SUPPORTED_EXT: &str = ".ZIP,.PPT,.PPTX,.TXT,.WMV,XLS,.RS,.JS,.CSS,.HTML,.WAV,.M4A,.M4B,.MP3,.FLAC,.OGG,.AAC,.WMA,.BMP,.GIF,.JPG,.JPEG,.PNG,.MPO,.ARW,.RAF,.TIF,.NEF,.MTS,.MP4,.MOV,.AVI,.PDF,";
