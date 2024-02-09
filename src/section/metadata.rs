use crate::Beatmap;
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

    pub fn get_metadata_title(&self) -> &str {
        &self.metadata.title
    }

    pub fn get_metadata_unicode_title(&self) -> &str {
        &self.metadata.title_unicode
    }

    pub fn get_metadata_artist(&self) -> &str {
        &self.metadata.artist
    }

    pub fn get_metadata_unicode_artist(&self) -> &str {
        &self.metadata.artist_unicode
    }

    pub fn get_metadata_creator(&self) -> &str {
        &self.metadata.creator
    }

    pub fn get_metadata_version(&self) -> &str {
        &self.metadata.version
    }

    pub fn get_metadata_source(&self) -> &str {
        &self.metadata.source
    }

    pub fn get_metadata_tags(&self) -> &str {
        &self.metadata.tags
    }

    pub fn get_metadata_beatmap_id(&self) -> i32 {
        self.metadata.beatmap_id
    }

    pub fn get_metadata_beatmap_set_id(&self) -> i32 {
        self.metadata.beatmap_set_id
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            source: String::new(),
            tags: String::new(),
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }
}

impl Metadata {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "Title" => self.title = value.to_string(),
            "TitleUnicode" => self.title_unicode = value.to_string(),
            "Artist" => self.artist = value.to_string(),
            "ArtistUnicode" => self.artist_unicode = value.to_string(),
            "Creator" => self.creator = value.to_string(),
            "Version" => self.version = value.to_string(),
            "Source" => self.source = value.to_string(),
            "Tags" => self.tags = value.to_string(),
            "BeatmapID" => self.beatmap_id = value.parse().unwrap(),
            "BeatmapSetID" => self.beatmap_set_id = value.parse().unwrap(),
            _ => {}
        }
    }
}
