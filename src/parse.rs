use std::io::Read;

use crate::{
    beatmap::Beatmap,
    sanitize::sanitize,
    section::{Colour, Difficulty, Editor, Events, General, HitObject, Metadata, TimingPoint},
    token::{DifficultyToken, EditorToken, EventsToken, GeneralToken, MetadataToken, Section},
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
    pub fn parse(osu_data: &str) -> std::io::Result<Beatmap> {
        let sanitized = sanitize(osu_data);

        let mut general_tokens: Vec<GeneralToken> = Vec::new();
        let mut editor_tokens: Vec<EditorToken> = Vec::new();
        let mut metadata_tokens: Vec<MetadataToken> = Vec::new();
        let mut difficulty_tokens: Vec<DifficultyToken> = Vec::new();
        let mut events_tokens: Vec<EventsToken> = Vec::new();

        let mut timing_points: Vec<TimingPoint> = Vec::new();
        let mut colours: Vec<Colour> = Vec::new();
        let mut hit_objects: Vec<HitObject> = Vec::new();

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
                                let split = line
                                    .splitn(2, ':')
                                    .map(|split| split.trim())
                                    .collect::<Vec<&str>>();

                                match section {
                                    Section::General => {
                                        general_tokens.push(GeneralToken::parse(&split[..])?);
                                    }
                                    Section::Editor => {
                                        editor_tokens.push(EditorToken::parse(&split[..])?);
                                    }
                                    Section::Metadata => {
                                        metadata_tokens.push(MetadataToken::parse(&split[..])?);
                                    }
                                    Section::Difficulty => {
                                        difficulty_tokens.push(DifficultyToken::parse(&split[..])?);
                                    }
                                    Section::Colours => colours.push(Colour::parse(&split[..])?),
                                    _ => {
                                        unreachable!("This should never happen");
                                    }
                                }

                                index += 1;
                            }
                        }
                    }
                    Section::Events | Section::TimingPoints => {
                        index += 1;
                        while let Some(line) = lines.get(index) {
                            if SECTIONS.contains(&line.as_str()) {
                                continue 'outer_loop;
                            } else {
                                let split = line
                                    .split(',')
                                    .map(|split| split.trim())
                                    .collect::<Vec<&str>>();

                                match section {
                                    Section::Events => {
                                        events_tokens.push(EventsToken::parse(&split[..])?);
                                    }
                                    Section::TimingPoints => {
                                        timing_points.push(TimingPoint::parse(&split[..])?);
                                    }
                                    _ => {
                                        unreachable!("This should never happen");
                                    }
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
            General::from(&general_tokens),
            Editor::parse(&editor_tokens)?,
            Metadata::from(&metadata_tokens),
            Difficulty::from(&difficulty_tokens),
            Events::from(&events_tokens),
            timing_points,
            colours,
            hit_objects,
        ))
    }

    pub fn parse_from_file(file: &str) -> std::io::Result<Beatmap> {
        let mut file = std::fs::File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Beatmap::parse(&contents)
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
        println!("{}", beatmap);
    }
}
