#[derive(Debug, PartialEq)]
pub enum Section {
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects,
}

impl Section {
    pub fn parse(s: &str) -> std::io::Result<Self> {
        match s {
            "[General]" => Ok(Section::General),
            "[Editor]" => Ok(Section::Editor),
            "[Metadata]" => Ok(Section::Metadata),
            "[Difficulty]" => Ok(Section::Difficulty),
            "[Events]" => Ok(Section::Events),
            "[TimingPoints]" => Ok(Section::TimingPoints),
            "[Colours]" => Ok(Section::Colours),
            "[HitObjects]" => Ok(Section::HitObjects),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid section : {}", s),
            )),
        }
    }
}
