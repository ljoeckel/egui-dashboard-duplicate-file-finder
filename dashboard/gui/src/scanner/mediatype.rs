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
#[derive(Debug, Clone)]
pub struct MediaGroup {
    pub name: String,
    pub media_types: Vec<MediaType>,
    pub selected: bool,
}
impl MediaGroup {
    fn new(name: &str, mut media_types: Vec<MediaType>, selected: bool) -> Self {
        // Setup selected for mediatypes
        for i in 0..media_types.len() {
            media_types[i].selected = selected;
        }

        Self {
            name: name.to_owned(),
            media_types,
            selected
        }
    }

    pub fn is_selected(&self, extension: &str) -> bool {
        if self.selected &&
            self.media_types
            .iter()
            .any(|t| t.extension.eq(extension) && t.selected) {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct MediaType {
    pub extension: String,
    pub description: String,
    pub selected: bool,
}

impl MediaType {
    pub fn new(extension: &str, description: &str) -> Self {
        Self {
            extension: extension.to_uppercase().to_owned(),
            description: description.to_owned(),
            selected: true,
        }
    }

    pub fn load_groups() -> Vec<MediaGroup> {
        vec![
            MediaGroup::new("Audio", audio_types(), true),
            MediaGroup::new("Document", document_types(), true),
            MediaGroup::new("Image", image_types(), true),
            MediaGroup::new("Video", video_types(), true),
            MediaGroup::new("IGNORED", ignore_types(), false),
        ]
    }
}

fn audio_types() -> Vec<MediaType> {
    vec![
        MediaType::new(".3gp", "MM container format can contain proprietary formats as AMR, AMR-WB or AMR-WB+, but also some open formats"),
        MediaType::new(".8svx", "The IFF-8SVX format for 8-bit sound samples, created by Electronic Arts in 1984 at the birth of the Amiga"),
        MediaType::new(".aa", "A low-bitrate audiobook container format with DRM, containing audio encoded as either MP3 or the ACELP speech codec"),
        MediaType::new(".aac", "The Advanced Audio Coding format is based on the MPEG-2 and MPEG-4 standards. AAC files are usually ADTS or ADIF containers"),
        MediaType::new(".aax", "An Audiobook format, which is a variable-bitrate (allowing high quality) M4B file encrypted with DRM. MPB contains AAC or ALAC encoded audio in an MPEG-4 container. (More details below.)"),
        MediaType::new(".act", "ACT is a lossy ADPCM 8 kbit/s compressed audio format recorded by most Chinese MP3 and MP4 players with a recording function, and voice recorders"),
        MediaType::new(".aiff", "A standard uncompressed CD-quality, audio file format used by Apple. Established 3 years prior to Microsoft's uncompressed version wav"),
        MediaType::new(".aif", "A standard uncompressed CD-quality, audio file format used by Apple. Established 3 years prior to Microsoft's uncompressed version wav"),
        MediaType::new(".alac", "An audio coding format developed by Apple Inc. for lossless data compression of digital music"),
        MediaType::new(".amr", "AMR-NB audio, used primarily for speech."),
        MediaType::new(".ape", "Monkeys Audio lossless audio compression format."),
        MediaType::new(".au", "The standard audio file format used by Sun, Unix and Java. The audio in au files can be PCM or compressed with the μ-law, a-law or G.729 codecs"),
        MediaType::new(".awb", "AMR-WB audio, used primarily for speech, same as the ITU-T's G.722.2 specification."),
        MediaType::new(".cda", "Format for cda files for Radio."),
        MediaType::new(".dss", "DSS files are an Olympus proprietary format. DSS files use a high compression rate, which reduces the file size and allows files to be copied and transferred quickly.[6] It allows additional data to be held in the file header"),
        MediaType::new(".dvf", "A Sony proprietary format for compressed voice files; commonly used by Sony dictation recorders"),
        MediaType::new(".flac", "A file format for the Free Lossless Audio Codec, an open-source lossless compression codec"),
        MediaType::new(".gsm", "Designed for telephony use in Europe, GSM is used to store telephone voice messages and conversations. With a bitrate of 13kbps, GSM files can compress and encode audio at telephone quality. [7] Note that WAV files can also be encoded with the GSM codec"),
        MediaType::new(".iklax", "An iKlax Media proprietary format, the iKlax format is a multi-track digital audio format allowing various actions on musical data, for instance on mixing and volumes arrangements"),
        MediaType::new(".ivs", "A proprietary version with DRM developed by 3D Solar UK Ltd for use in music downloaded from their Tronme Music Store and interactive music and video player"),
        MediaType::new(".m4a", "An audio-only MPEG-4 file, used by Apple for unprotected music downloaded from their iTunes Music Store. Audio within the m4a file is typically encoded with AAC, although lossless ALAC may also be used"),
        MediaType::new(".m4b", "An Audiobook / podcast extension with AAC or ALAC encoded audio in an MPEG-4 container. Both M4A and M4B formats can contain metadata including chapter markers, images, and hyperlinks, but M4B allows 'bookmarks' (remembering the last listening spot), whereas M4A does not.[8]"),
        MediaType::new(".m4p", "A version of AAC with proprietary DRM developed by Apple for use in music downloaded from their iTunes Music Store and their music streaming service known as Apple Music"),
        MediaType::new(".mmf", "A Samsung audio format that is used in ringtones. Developed by Yamaha (SMAF stands for 'Synthetic music Mobile Application Format', and is a multimedia data format invented by the Yamaha Corporation, .mmf file format)"),
        MediaType::new(".movpkg", "An Apple audio format primarily used for Lossless and Hi-Res audio files through Apple Music. Also used for storing Apple TV videos"),
        MediaType::new(".mp3", "MPEG Layer III Audio"),
        MediaType::new(".mpc", "Musepack or MPC (formerly known as MPEGplus, MPEG+ or MP+) is an open source lossy audio codec, specifically optimized for transparent compression of stereo audio at bitrates of 160–180 kbit/s"),
        MediaType::new(".msv", "A Sony proprietary format for Memory Stick compressed voice files."),
        MediaType::new(".nmf", "NICE Media Player audio file"),
        MediaType::new(".ogg", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format"),
        MediaType::new(".oga", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format"),
        MediaType::new(".mogg", "A free, open source container format supporting a variety of formats, the most popular of which is the audio format Vorbis. Vorbis offers compression similar to MP3 but is less popular. Mogg, the 'Multi-Track-Single-Logical-Stream Ogg-Vorbis', is the multi-channel or multi-track Ogg file format"),
        MediaType::new(".opus", "A lossy audio compression format developed by the Internet Engineering Task Force (IETF) and made especially suitable for interactive real-time applications over the Internet. As an open format standardised through RFC 6716, a reference implementation is provided under the 3-clause BSD license"),
        MediaType::new(".ra", "A RealAudio format designed for streaming audio over the Internet. The .ra format allows files to be stored in a self-contained fashion on a computer, with all of the audio data contained inside the file itself"),
        MediaType::new(".rm", "A RealAudio format designed for streaming audio over the Internet. The .ra format allows files to be stored in a self-contained fashion on a computer, with all of the audio data contained inside the file itself"),
        MediaType::new(".raw", "A raw file can contain audio in any format but is usually used with PCM audio data. It is rarely used except for technical tests"),
        MediaType::new(".rf64", "One successor to the Wav format, overcoming the 4GiB size limitation."),
        MediaType::new(".sln", "Signed Linear PCM format used by Asterisk. Prior to v.10 the standard formats were 16-bit Signed Linear PCM sampled at 8 kHz and at 16 kHz. With v.10 many more sampling rates were added.[9]"),
        MediaType::new(".tta", "The True Audio, real-time lossless audio codec."),
        MediaType::new(".voc", "The file format consists of a 26-byte header and a series of subsequent data blocks containing the audio information"),
        MediaType::new(".vox", "The vox format most commonly uses the Dialogic ADPCM (Adaptive Differential Pulse Code Modulation) codec. Similar to other ADPCM formats, it compresses to 4-bits. Vox format files are similar to wave files except that the vox files contain no information about the file itself so the codec sample rate and number of channels must first be specified in order to play a vox file"),
        MediaType::new(".wav", "Standard audio file container format used mainly in Windows PCs. Commonly used for storing uncompressed (PCM), CD-quality sound files, which means that they can be large in size—around 10 MB per minute. Wave files can also contain data encoded with a variety of (lossy) codecs to reduce the file size (for example the GSM or MP3 formats). Wav files use a RIFF structure"),
        MediaType::new(".wma", "Windows Media Audio format, created by Microsoft. Designed with DRM abilities for copy protection"),
        MediaType::new(".wv", "Format for wavpack files"),
        MediaType::new(".webm", "Royalty-free format created for HTML video"),
    ]
}

fn image_types() -> Vec<MediaType> {
    vec![
        MediaType::new(".png", "Portable Network Graphic"),
        MediaType::new(".png", "Portable Network Graphic"),
        MediaType::new(".jpg", "JPEG Image"),
        MediaType::new(".jpeg", "JPEG Image"),
        MediaType::new(".tiff", "Tagged Image Format"),
        MediaType::new(".bmp", "Bitmap Image"),
    ]
}

fn video_types() -> Vec<MediaType> {
    vec![
        MediaType::new(".m4v", "Apple iTunes Video file"),
        MediaType::new(".mp4", "MPEG4 Video"),
        MediaType::new(".vob", "Multimedia container format can contain proprietary formats as AMR, AMR-WB or AMR-WB+, but also some open formats"),
    ]
}

fn document_types() -> Vec<MediaType> {
    vec![
        MediaType::new(".doc", "Microsoft Word Document (Old format)"),
        MediaType::new(".docx", "Microsoft Word Document"),
        MediaType::new(".pdf", "Adobes multi-platform document format"),
        MediaType::new(".txt", "Plain text file"),
    ]
}

fn ignore_types() -> Vec<MediaType> {
    vec![
        MediaType::new("._", "Files with this extension will be ignored"),
        MediaType::new(".A", "Files with this extension will be ignored"),
        MediaType::new(".BAK", "Files with this extension will be ignored"),
        MediaType::new(".BIN", "Files with this extension will be ignored"),
        MediaType::new(".DS_STORE", "Files with this extension will be ignored"),
        MediaType::new(".DS_SSTORE", "Files with this extension will be ignored"),
        MediaType::new(".GITIGNORE", "Files with this extension will be ignored"),
        MediaType::new(".IDX", "Files with this extension will be ignored"),
        MediaType::new(".LOCK", "Files with this extension will be ignored"),
        MediaType::new(".LOG", "Files with this extension will be ignored"),
        MediaType::new(".M3U", "Files with this extension will be ignored"),
        MediaType::new(".MF", "Files with this extension will be ignored"),
        MediaType::new(".NFO", "Files with this extension will be ignored"),
        MediaType::new(".O", "Files with this extension will be ignored"),
        MediaType::new(".RLIB", "Files with this extension will be ignored"),
        MediaType::new(".RMETA", "Files with this extension will be ignored"),
        MediaType::new(".RTF", "Files with this extension will be ignored"),
        MediaType::new(".SFV", "Files with this extension will be ignored"),
        MediaType::new(".TIMESTAMP", "Files with this extension will be ignored"),
        MediaType::new(".URL", "Files with this extension will be ignored"),
        MediaType::new(".WPL", "Files with this extension will be ignored"),
    ]
}


