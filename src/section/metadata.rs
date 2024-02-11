use crate::Beatmap;
#[derive(Debug)]
pub struct Metadata {
    title: Option<String>,
    title_unicode: Option<String>,
    artist: Option<String>,
    artist_unicode: Option<String>,
    creator: Option<String>,
    version: Option<String>,
    source: Option<String>,
    tags: Option<String>,
    beatmap_id: Option<i32>,
    beatmap_set_id: Option<i32>,
}

impl Metadata {
    pub fn change_name(&mut self, name: &str) {
        self.title = Some(name.to_string());
    }

    pub fn change_unicode_name(&mut self, name: &str) {
        self.title_unicode = Some(name.to_string());
    }

    pub fn change_artist(&mut self, artist: &str) {
        self.artist = Some(artist.to_string());
    }

    pub fn change_unicode_artist(&mut self, artist: &str) {
        self.artist_unicode = Some(artist.to_string());
    }

    pub fn change_creator(&mut self, creator: &str) {
        self.creator = Some(creator.to_string());
    }

    pub fn change_version(&mut self, version: &str) {
        self.version = Some(version.to_string());
    }

    pub fn change_source(&mut self, source: &str) {
        self.source = Some(source.to_string());
    }

    pub fn change_tags(&mut self, tags: &str) {
        self.tags = Some(tags.to_string());
    }

    pub fn change_beatmap_id(&mut self, id: i32) {
        self.beatmap_id = Some(id);
    }

    pub fn change_beatmap_set_id(&mut self, id: i32) {
        self.beatmap_set_id = Some(id);
    }
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = String::from("[Metadata]\n");
        if let Some(title) = &self.title {
            display_string.push_str(&format!("Title:{}\n", title));
        }
        if let Some(title_unicode) = &self.title_unicode {
            display_string.push_str(&format!("TitleUnicode:{}\n", title_unicode));
        }
        if let Some(artist) = &self.artist {
            display_string.push_str(&format!("Artist:{}\n", artist));
        }
        if let Some(artist_unicode) = &self.artist_unicode {
            display_string.push_str(&format!("ArtistUnicode:{}\n", artist_unicode));
        }
        if let Some(creator) = &self.creator {
            display_string.push_str(&format!("Creator:{}\n", creator));
        }
        if let Some(version) = &self.version {
            display_string.push_str(&format!("Version:{}\n", version));
        }
        if let Some(source) = &self.source {
            display_string.push_str(&format!("Source:{}\n", source));
        }
        if let Some(tags) = &self.tags {
            display_string.push_str(&format!("Tags:{}\n", tags));
        }
        if let Some(beatmap_id) = &self.beatmap_id {
            display_string.push_str(&format!("BeatmapID:{}\n", beatmap_id));
        }
        if let Some(beatmap_set_id) = &self.beatmap_set_id {
            display_string.push_str(&format!("BeatmapSetID:{}\n", beatmap_set_id));
        }
        write!(f, "{}", display_string)
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
        match &self.metadata.title {
            Some(title) => title,
            None => "",
        }
    }

    pub fn get_metadata_unicode_title(&self) -> &str {
        match &self.metadata.title_unicode {
            Some(title) => title,
            None => "",
        }
    }

    pub fn get_metadata_artist(&self) -> &str {
        match &self.metadata.artist {
            Some(artist) => artist,
            None => "",
        }
    }

    pub fn get_metadata_unicode_artist(&self) -> &str {
        match &self.metadata.artist_unicode {
            Some(artist) => artist,
            None => "",
        }
    }

    pub fn get_metadata_creator(&self) -> &str {
        match &self.metadata.creator {
            Some(creator) => creator,
            None => "",
        }
    }

    pub fn get_metadata_version(&self) -> &str {
        match &self.metadata.version {
            Some(version) => version,
            None => "",
        }
    }

    pub fn get_metadata_source(&self) -> &str {
        match &self.metadata.source {
            Some(source) => source,
            None => "",
        }
    }

    pub fn get_metadata_tags(&self) -> &str {
        match &self.metadata.tags {
            Some(tags) => tags,
            None => "",
        }
    }

    pub fn get_metadata_beatmap_id(&self) -> i32 {
        match &self.metadata.beatmap_id {
            Some(id) => *id,
            None => 0,
        }
    }

    pub fn get_metadata_beatmap_set_id(&self) -> i32 {
        match &self.metadata.beatmap_set_id {
            Some(id) => *id,
            None => 0,
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            title: None,
            title_unicode: None,
            artist: None,
            artist_unicode: None,
            creator: None,
            version: None,
            source: None,
            tags: None,
            beatmap_id: None,
            beatmap_set_id: None,
        }
    }
}

impl Metadata {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "Title" => self.title = Some(value.to_string()),
            "TitleUnicode" => self.title_unicode = Some(value.to_string()),
            "Artist" => self.artist = Some(value.to_string()),
            "ArtistUnicode" => self.artist_unicode = Some(value.to_string()),
            "Creator" => self.creator = Some(value.to_string()),
            "Version" => self.version = Some(value.to_string()),
            "Source" => self.source = Some(value.to_string()),
            "Tags" => self.tags = Some(value.to_string()),
            "BeatmapID" => self.beatmap_id = Some(value.parse().unwrap()),
            "BeatmapSetID" => self.beatmap_set_id = Some(value.parse().unwrap()),
            _ => {}
        }
    }
}
