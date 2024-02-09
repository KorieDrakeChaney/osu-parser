use std::{ffi::OsString, io::Read, path::PathBuf};

use crate::{
    beatmap::Beatmap,
    sanitize::sanitize,
    section::{
        Colour, Command, Difficulty, Editor, Events, General, HitObject, Metadata, Storyboard,
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
    fn parse(osu_data: &str, directory: OsString) -> std::io::Result<Beatmap> {
        let sanitized = sanitize(osu_data);

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
                                let split = line
                                    .split(',')
                                    .map(|split| split.trim())
                                    .collect::<Vec<&str>>();

                                if split[0] == "Sprite" || split[0] == "Animation" {
                                    index += 1;
                                    let mut commands: Vec<Command> = Vec::new();
                                    while let Some(line) = lines.get(index) {
                                        match Command::parse(line.as_str()) {
                                            Ok(command) => {
                                                commands.push(command);
                                                index += 1;
                                            }
                                            Err(_) => {
                                                index -= 1;
                                                break;
                                            }
                                        }
                                    }

                                    events.push_storyboard(Storyboard::parse(
                                        line.as_str(),
                                        commands,
                                    )?);
                                } else {
                                    events.parse_value(line.as_str());
                                }

                                index += 1;
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
            directory,
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

    pub fn parse_from_file(file: &str) -> std::io::Result<Beatmap> {
        let path = PathBuf::from(file);
        let directory = OsString::from(path.parent().unwrap());
        let mut file = std::fs::File::open(path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Beatmap::parse(&contents, directory)
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test() {
        let mut beatmap: Beatmap = Beatmap::parse_from_file("test.osu").unwrap();
        beatmap.change_metadata_title("@KorieDrakeChaney was here");
        beatmap.save("test2.osu");
    }
}
