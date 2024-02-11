use std::{ffi::OsString, io::Read, path::Path};

use crate::{
    beatmap::Beatmap,
    sanitize::sanitize,
    section::{
        Colour, Command, Difficulty, Editor, Events, General, HitObject, Metadata, OsuStoryboard,
        TimingPoint,
    },
    token::Section,
};

const SECTIONS: [&'static str; 8] = [
    "[General]",
    "[Editor]",
    "[Metadata]",
    "[Difficulty]",
    "[Events]",
    "[TimingPoints]",
    "[Colours]",
    "[HitObjects]",
];

impl Beatmap {
    fn parse(osu_data: &str, directory: OsString, file_name: &str) -> std::io::Result<Beatmap> {
        let sanitized = sanitize(osu_data);

        let mut version = 14;

        let mut timing_points: Vec<TimingPoint> = Vec::new();
        let mut colours: Vec<Colour> = Vec::new();
        let mut hit_objects: Vec<HitObject> = Vec::new();

        let mut general: General = General::default();
        let mut editor: Editor = Editor::default();
        let mut metadata: Metadata = Metadata::default();
        let mut difficulty: Difficulty = Difficulty::default();
        let mut events: Events = Events::default();

        let lines = sanitized
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let mut index = 0;

        'outer_loop: while let Some(line) = lines.get(index) {
            if index == 0 {
                if line.contains("osu file format v") {
                    version = line.split('v').collect::<Vec<&str>>()[1]
                        .parse::<u8>()
                        .unwrap();
                }
            }
            if let Ok(section) = Section::parse(line) {
                match section {
                    Section::General
                    | Section::Editor
                    | Section::Metadata
                    | Section::Difficulty
                    | Section::Colours => {
                        index += 1;
                        while let Some(line) = lines.get(index) {
                            if SECTIONS.contains(&line.as_str()) {
                                continue 'outer_loop;
                            } else {
                                match section {
                                    Section::General => {
                                        general.parse_value(line.as_str());
                                    }
                                    Section::Editor => {
                                        editor.parse_value(line.as_str());
                                    }
                                    Section::Metadata => {
                                        metadata.parse_value(line.as_str());
                                    }
                                    Section::Difficulty => {
                                        difficulty.parse_value(line.as_str());
                                    }
                                    Section::Colours => {
                                        colours.push(Colour::parse(line.as_str())?);
                                    }
                                    _ => {
                                        unreachable!("This should never happen");
                                    }
                                }

                                index += 1;
                            }
                        }
                    }
                    Section::TimingPoints => {
                        index += 1;
                        while let Some(line) = lines.get(index) {
                            if SECTIONS.contains(&line.as_str()) {
                                continue 'outer_loop;
                            } else {
                                let split = line
                                    .split(',')
                                    .map(|split| split.trim())
                                    .collect::<Vec<&str>>();

                                timing_points.push(TimingPoint::parse(&split[..])?);

                                index += 1;
                            }
                        }
                    }
                    Section::Events => {
                        index += 1;

                        while let Some(line) = lines.get(index) {
                            if SECTIONS.contains(&line.as_str()) {
                                continue 'outer_loop;
                            } else {
                                if let Ok(mut storyboard) = OsuStoryboard::parse(line.as_str()) {
                                    index += 1;

                                    'command_loop: while let Some(potential_command) =
                                        lines.get(index)
                                    {
                                        if let Ok(command) = Command::parse(&potential_command) {
                                            storyboard.add_command(command);
                                            index += 1;
                                        } else {
                                            break 'command_loop;
                                        }
                                    }
                                    events.push_storyboard(storyboard);
                                } else {
                                    events.parse_value(line.as_str());
                                    index += 1;
                                }
                            }
                        }
                    }
                    Section::HitObjects => {
                        index += 1;
                        while let Some(line) = lines.get(index) {
                            let split = line
                                .split(',')
                                .map(|split| split.trim())
                                .collect::<Vec<&str>>();

                            match section {
                                Section::HitObjects => {
                                    hit_objects.push(HitObject::parse(&split[..])?);
                                }
                                _ => {
                                    unreachable!("This should never happen");
                                }
                            }

                            index += 1;
                        }
                    }
                }
            }

            index += 1;
        }

        Ok(Beatmap::new(
            file_name.to_string(),
            directory,
            version,
            general,
            editor,
            metadata,
            difficulty,
            events,
            timing_points,
            colours,
            hit_objects,
        ))
    }

    pub fn parse_file(file: &str) -> std::io::Result<Beatmap> {
        let path = Path::new(file);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let directory = OsString::from(path.parent().unwrap());
        let mut file = std::fs::File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Beatmap::parse(&contents, directory, file_name)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut beatmap: Beatmap = Beatmap::parse_file("beatmap.osu").unwrap();
        beatmap.change_metadata_title("@KorieDrakeChaney was here");
        beatmap.save_with_name("test_beatmap.osu");
    }
}
