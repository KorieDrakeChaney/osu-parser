use crate::{token::MetadataToken, Beatmap};
#[derive(Debug)]
pub struct Metadata {
    title: String,
    title_unicode: String,
    artist: String,
    artist_unicode: String,
    creator: String,
    version: String,
    source: String,
    tags: String,
    beatmap_id: i32,
    beatmap_set_id: i32,
}

impl Metadata {
    pub fn change_name(&mut self, name: &str) {
        self.title = name.to_string();
    }

    pub fn change_unicode_name(&mut self, name: &str) {
        self.title_unicode = name.to_string();
    }

    pub fn change_artist(&mut self, artist: &str) {
        self.artist = artist.to_string();
    }

    pub fn change_unicode_artist(&mut self, artist: &str) {
        self.artist_unicode = artist.to_string();
    }

    pub fn change_creator(&mut self, creator: &str) {
        self.creator = creator.to_string();
    }

    pub fn change_version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    pub fn change_source(&mut self, source: &str) {
        self.source = source.to_string();
    }

    pub fn change_tags(&mut self, tags: &str) {
        self.tags = tags.to_string();
    }

    pub fn change_beatmap_id(&mut self, id: i32) {
        self.beatmap_id = id;
    }

    pub fn change_beatmap_set_id(&mut self, id: i32) {
        self.beatmap_set_id = id;
    }
}

impl From<&Vec<MetadataToken>> for Metadata {
    fn from(tokens: &Vec<MetadataToken>) -> Self {
        let mut title = String::new();
        let mut title_unicode = String::new();
        let mut artist = String::new();
        let mut artist_unicode = String::new();
        let mut creator = String::new();
        let mut version = String::new();
        let mut source = String::new();
        let mut tags = String::new();
        let mut beatmap_id = 0;
        let mut beatmap_set_id = 0;

        for token in tokens {
            match token {
                MetadataToken::Title(t) => title = t.to_string(),
                MetadataToken::TitleUnicode(t) => title_unicode = t.to_string(),
                MetadataToken::Artist(a) => artist = a.to_string(),
                MetadataToken::ArtistUnicode(a) => artist_unicode = a.to_string(),
                MetadataToken::Creator(c) => creator = c.to_string(),
                MetadataToken::Version(v) => version = v.to_string(),
                MetadataToken::Source(s) => source = s.to_string(),
                MetadataToken::Tags(t) => tags = t.to_string(),
                MetadataToken::BeatmapID(id) => beatmap_id = *id,
                MetadataToken::BeatmapSetID(id) => beatmap_set_id = *id,
            }
        }

        Metadata {
            title,
            title_unicode,
            artist,
            artist_unicode,
            creator,
            version,
            source,
            tags,
            beatmap_id,
            beatmap_set_id,
        }
    }
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Metadata]\n")?;
        write!(f, "Title: {}\n", self.title)?;
        write!(f, "TitleUnicode: {}\n", self.title_unicode)?;
        write!(f, "Artist: {}\n", self.artist)?;
        write!(f, "ArtistUnicode: {}\n", self.artist_unicode)?;
        write!(f, "Creator: {}\n", self.creator)?;
        write!(f, "Version: {}\n", self.version)?;
        write!(f, "Source: {}\n", self.source)?;
        write!(f, "Tags: {}\n", self.tags)?;
        write!(f, "BeatmapID: {}\n", self.beatmap_id)?;
        write!(f, "BeatmapSetID: {}\n", self.beatmap_set_id)
    }
}

impl Beatmap {
    pub fn change_metadata_title(&mut self, title: &str) {
        self.metadata.change_name(title);
    }

    pub fn change_metadata_unicode_title(&mut self, title: &str) {
        self.metadata.change_unicode_name(title);
    }

    pub fn change_metadata_artist(&mut self, artist: &str) {
        self.metadata.change_artist(artist);
    }

    pub fn change_metadata_unicode_artist(&mut self, artist: &str) {
        self.metadata.change_unicode_artist(artist);
    }

    pub fn change_metadata_creator(&mut self, creator: &str) {
        self.metadata.change_creator(creator);
    }

    pub fn change_metadata_version(&mut self, version: &str) {
        self.metadata.change_version(version);
    }

    pub fn change_metadata_source(&mut self, source: &str) {
        self.metadata.change_source(source);
    }

    pub fn change_metadata_tags(&mut self, tags: &str) {
        self.metadata.change_tags(tags);
    }

    pub fn change_metadata_beatmap_id(&mut self, id: i32) {
        self.metadata.change_beatmap_id(id);
    }

    pub fn change_metadata_beatmap_set_id(&mut self, id: i32) {
        self.metadata.change_beatmap_set_id(id);
    }
}
