use std::fmt::format;

#[derive(Debug, PartialEq)]
pub enum GeneralToken {
    AudioFilename(String),
    AudioLeadIn(i32),
    PreviewTime(i32),
    Countdown(i32),
    SampleSet(String),
    StackLeniency(f32),
    Mode(i32),
    LetterboxInBreaks(bool),
    UseSkinSprites(bool),
    OverlayPosition(String),
    SkinPreference(String),
    EpilepsyWarning(bool),
    CountdownOffset(i32),
    SpecialStyle(bool),
    WidescreenStoryboard(bool),
    SamplesMatchSpeed(bool),

    AudioHash(String),
    StoryFireInFront(bool),
    AlwaysShowPlayfield(bool),
}

impl GeneralToken {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        match s[0] {
            "AudioFilename" => Ok(GeneralToken::AudioFilename(s[1].to_string())),
            "AudioLeadIn" => Ok(GeneralToken::AudioLeadIn(s[1].parse().unwrap())),
            "PreviewTime" => Ok(GeneralToken::PreviewTime(s[1].parse().unwrap())),
            "Countdown" => Ok(GeneralToken::Countdown(s[1].parse().unwrap())),
            "SampleSet" => Ok(GeneralToken::SampleSet(s[1].to_string())),
            "StackLeniency" => Ok(GeneralToken::StackLeniency(s[1].parse().unwrap())),
            "Mode" => Ok(GeneralToken::Mode(s[1].parse().unwrap())),
            "LetterboxInBreaks" => Ok(GeneralToken::LetterboxInBreaks(s[1] == "1")),
            "UseSkinSprites" => Ok(GeneralToken::UseSkinSprites(s[1] == "1")),
            "OverlayPosition" => Ok(GeneralToken::OverlayPosition(s[1].to_string())),
            "SkinPreference" => Ok(GeneralToken::SkinPreference(s[1].to_string())),
            "EpilepsyWarning" => Ok(GeneralToken::EpilepsyWarning(s[1] == "1")),
            "CountdownOffset" => Ok(GeneralToken::CountdownOffset(s[1].parse().unwrap())),
            "SpecialStyle" => Ok(GeneralToken::SpecialStyle(s[1] == "1")),
            "WidescreenStoryboard" => Ok(GeneralToken::WidescreenStoryboard(s[1] == "1")),
            "SamplesMatchSpeed" => Ok(GeneralToken::SamplesMatchSpeed(s[1] == "1")),
            "AudioHash" => Ok(GeneralToken::AudioHash(s[1].to_string())),
            "StoryFireInFront" => Ok(GeneralToken::StoryFireInFront(s[1] == "1")),
            "AlwaysShowPlayfield" => Ok(GeneralToken::AlwaysShowPlayfield(s[1] == "1")),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid General token: {}", s[0]),
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EditorToken {
    Bookmarks(String),
    DistanceSpacing(f32),
    BeatDivisor(i32),
    GridSize(i32),
    TimelineZoom(f32),
}

impl EditorToken {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        match s[0] {
            "Bookmarks" => Ok(EditorToken::Bookmarks(s[1].to_string())),
            "DistanceSpacing" => Ok(EditorToken::DistanceSpacing(s[1].parse().unwrap())),
            "BeatDivisor" => Ok(EditorToken::BeatDivisor(s[1].parse().unwrap())),
            "GridSize" => Ok(EditorToken::GridSize(s[1].parse().unwrap())),
            "TimelineZoom" => Ok(EditorToken::TimelineZoom(s[1].parse().unwrap())),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid Editor token: {}", s[0]),
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MetadataToken {
    Title(String),
    TitleUnicode(String),
    Artist(String),
    ArtistUnicode(String),
    Creator(String),
    Version(String),
    Source(String),
    Tags(String),
    BeatmapID(i32),
    BeatmapSetID(i32),
}

impl MetadataToken {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        match s[0] {
            "Title" => Ok(MetadataToken::Title(s[1].to_string())),
            "TitleUnicode" => Ok(MetadataToken::TitleUnicode(s[1].to_string())),
            "Artist" => Ok(MetadataToken::Artist(s[1].to_string())),
            "ArtistUnicode" => Ok(MetadataToken::ArtistUnicode(s[1].to_string())),
            "Creator" => Ok(MetadataToken::Creator(s[1].to_string())),
            "Version" => Ok(MetadataToken::Version(s[1].to_string())),
            "Source" => Ok(MetadataToken::Source(s[1].to_string())),
            "Tags" => Ok(MetadataToken::Tags(s[1].to_string())),
            "BeatmapID" => Ok(MetadataToken::BeatmapID(s[1].parse().unwrap())),
            "BeatmapSetID" => Ok(MetadataToken::BeatmapSetID(s[1].parse().unwrap())),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid Metadata token: {}", s[0]),
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DifficultyToken {
    HPDrainRate(f32),
    CircleSize(f32),
    OverallDifficulty(f32),
    ApproachRate(f32),
    SliderMultiplier(f32),
    SliderTickRate(f32),
}

impl DifficultyToken {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        match s[0] {
            "HPDrainRate" => Ok(DifficultyToken::HPDrainRate(s[1].parse().unwrap())),
            "CircleSize" => Ok(DifficultyToken::CircleSize(s[1].parse().unwrap())),
            "OverallDifficulty" => Ok(DifficultyToken::OverallDifficulty(s[1].parse().unwrap())),
            "ApproachRate" => Ok(DifficultyToken::ApproachRate(s[1].parse().unwrap())),
            "SliderMultiplier" => Ok(DifficultyToken::SliderMultiplier(s[1].parse().unwrap())),
            "SliderTickRate" => Ok(DifficultyToken::SliderTickRate(s[1].parse().unwrap())),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid Difficulty token: {}", s[0]),
            )),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum EventsToken {
    Event(String, i32, String),
    Background(String, i32, i32),
    Video(i32, String, i32, i32),
    Break(i32, i32),
    Storyboard(String),
}

impl EventsToken {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        match s[0] {
            "0" => {
                if s[1] == "0" {
                    if s.len() == 3 {
                        Ok(EventsToken::Background(s[2].to_string(), 0, 0))
                    } else {
                        Ok(EventsToken::Background(
                            s[2].to_string(),
                            s[3].parse().unwrap(),
                            s[4].parse().unwrap(),
                        ))
                    }
                } else {
                    Ok(EventsToken::Event(
                        s[1].to_string(),
                        s[2].parse().unwrap(),
                        s[3].to_string(),
                    ))
                }
            }
            "1" | "Video" => {
                if s.len() == 3 {
                    Ok(EventsToken::Video(
                        s[1].parse().unwrap(),
                        s[2].to_string(),
                        0,
                        0,
                    ))
                } else {
                    Ok(EventsToken::Video(
                        s[1].parse().unwrap(),
                        s[2].to_string(),
                        s[3].parse().unwrap(),
                        s[4].parse().unwrap(),
                    ))
                }
            }
            "2" | "Break" => Ok(EventsToken::Break(
                s[1].parse().unwrap(),
                s[2].parse().unwrap(),
            )),
            _ => Ok(EventsToken::Event(
                s[0].to_string(),
                s[1].parse().unwrap(),
                s[2].to_string(),
            )),
        }
    }
}

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
