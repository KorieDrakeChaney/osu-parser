use std::{ffi::OsString, path::PathBuf};

use crate::section::{
    Colour, Difficulty, Editor, Events, General, HitObject, Metadata, TimingPoint,
};

#[derive(Debug)]
pub struct Beatmap {
    file_name: String,
    directory: OsString,
    version: u8,
    pub(crate) general: General,
    pub(crate) difficulty: Difficulty,
    pub(crate) metadata: Metadata,
    pub(crate) editor: Editor,
    pub(crate) events: Events,
    pub(crate) timing_points: Vec<TimingPoint>,
    pub(crate) colours: Vec<Colour>,
    pub(crate) hit_objects: Vec<HitObject>,
}

impl Beatmap {
    pub(crate) fn new(
        file_name: String,
        directory: OsString,
        version: u8,
        general: General,
        editor: Editor,
        metadata: Metadata,
        difficulty: Difficulty,
        events: Events,
        timing_points: Vec<TimingPoint>,
        colours: Vec<Colour>,
        hit_objects: Vec<HitObject>,
    ) -> Self {
        Beatmap {
            file_name,
            directory,
            version,
            general,
            difficulty,
            metadata,
            editor,
            events,
            timing_points,
            colours,
            hit_objects,
        }
    }

    pub fn save_with_name(&self, name: &str) {
        std::fs::write(name, self.to_string()).unwrap();
    }

    pub fn save_to_directory_with_name(&self, name: &str) -> std::io::Result<()> {
        std::fs::write(PathBuf::from(&self.directory).join(name), self.to_string())?;

        Ok(())
    }

    pub fn save(&self) {
        self.save_with_name(self.get_file_name());
    }

    pub fn save_to_directory(&self) -> std::io::Result<()> {
        self.save_to_directory_with_name(self.get_file_name())
    }

    pub fn get_hit_objects(&self) -> &Vec<HitObject> {
        &self.hit_objects
    }

    pub fn get_timing_points(&self) -> &Vec<TimingPoint> {
        &self.timing_points
    }

    pub fn get_directory(&self) -> &OsString {
        &self.directory
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
}

impl std::fmt::Display for Beatmap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "osu file format v{}\n\n{}\n\n{}\n\n{}\n\n{}\n\n{}",
            self.version, self.general, self.editor, self.metadata, self.difficulty, self.events,
        );

        if self.colours.len() > 0 {
            display_string += "\n\n[Colours]\n";
            for colour in &self.colours {
                display_string += &format!("{}", colour);
            }
        }

        if self.timing_points.len() > 0 {
            display_string += "\n\n[TimingPoints]\n";
            for timing_point in &self.timing_points {
                display_string += &format!("{}", timing_point);
            }
        }

        if self.hit_objects.len() > 0 {
            display_string += "\n\n[HitObjects]\n";
            for hit_object in &self.hit_objects {
                display_string += &format!("{}", hit_object);
            }
        }

        write!(f, "{}", display_string)
    }
}

impl PartialEq for Beatmap {
    fn eq(&self, other: &Self) -> bool {
        self.get_metadata_beatmap_id() == other.get_metadata_beatmap_id()
            && self.get_metadata_version() == other.get_metadata_version()
    }
}

impl Eq for Beatmap {}
