use std::ffi::OsString;

use crate::token::GeneralToken;
#[derive(Debug)]
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

impl std::fmt::Display for SampleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SampleSet::Normal => write!(f, "Normal"),
            SampleSet::Soft => write!(f, "Soft"),
            SampleSet::Drum => write!(f, "Drum"),
        }
    }
}

impl Default for SampleSet {
    fn default() -> Self {
        SampleSet::Normal
    }
}

impl From<&str> for SampleSet {
    fn from(s: &str) -> Self {
        match s {
            "Normal" => SampleSet::Normal,
            "Soft" => SampleSet::Soft,
            "Drum" => SampleSet::Drum,
            _ => SampleSet::Normal,
        }
    }
}
#[derive(Debug)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

impl std::fmt::Display for OverlayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OverlayPosition::NoChange => write!(f, "NoChange"),
            OverlayPosition::Below => write!(f, "Below"),
            OverlayPosition::Above => write!(f, "Above"),
        }
    }
}

impl Default for OverlayPosition {
    fn default() -> Self {
        OverlayPosition::NoChange
    }
}

impl From<&str> for OverlayPosition {
    fn from(s: &str) -> Self {
        match s {
            "NoChange" => OverlayPosition::NoChange,
            "Below" => OverlayPosition::Below,
            "Above" => OverlayPosition::Above,
            _ => OverlayPosition::NoChange,
        }
    }
}
#[derive(Debug)]
pub struct General {
    /// Location of the audio file
    pub audio_file: OsString,
    /// Milliseconds of silence before the audio starts playing
    pub audio_lead_in: i32,
    /// Time in milliseconds when the audio preview should start
    pub preview_time: i32,
    /// Speed of the coundown before the first hit object
    pub countdown: i32,
    /// Sample set that will be used if timing points do not override it
    pub sample_set: SampleSet,
    /// Multiplier for teh threshold in the time where hit objects placed close together stack
    pub stack_leniency: f32,
    /// Game mode
    pub mode: i32,
    /// Whether or not breaks have a letterboxing effect
    pub letterbox_in_breaks: bool,
    /// Whether or not the storyboard can use the user's skin images
    pub use_skin_sprites: bool,
    ///  Draw order of hit circle overlays compared to hit numbers
    pub overlay_position: OverlayPosition,
    /// Preferred skin to use during gameplay
    pub skin_preference: String,
    /// Epilepsy warning
    pub epilepsy_warning: bool,
    /// Time in beats that the countdown starts before the first hit object
    pub countdown_offset: i32,
    /// Where or not the "N+1" style key layout is used for osu!mania
    pub special_style: bool,
    /// Where or not the storyboard allows widescreen viewing
    pub widescreen_storyboard: bool,
    /// Where or not sound samples will change rate when playing with speed-changing mods
    pub samples_match_speed: bool,
}

impl From<&Vec<GeneralToken>> for General {
    fn from(tokens: &Vec<GeneralToken>) -> Self {
        let mut audio_file = OsString::new();
        let mut audio_lead_in = 0;
        let mut preview_time = -1;
        let mut countdown = 1;
        let mut sample_set = SampleSet::default();
        let mut stack_leniency = 0.7;
        let mut mode = 0;
        let mut letterbox_in_breaks = false;
        let mut use_skin_sprites = false;
        let mut overlay_position = OverlayPosition::default();
        let mut skin_preference = String::new();
        let mut epilepsy_warning = false;
        let mut countdown_offset = 0;
        let mut special_style = false;
        let mut widescreen_storyboard = false;
        let mut samples_match_speed = false;

        for token in tokens {
            match token {
                GeneralToken::AudioFilename(s) => audio_file = OsString::from(s),
                GeneralToken::AudioLeadIn(i) => audio_lead_in = *i,
                GeneralToken::PreviewTime(i) => preview_time = *i,
                GeneralToken::Countdown(i) => countdown = *i,
                GeneralToken::SampleSet(s) => sample_set = SampleSet::from(s.as_str()),
                GeneralToken::StackLeniency(f) => stack_leniency = *f,
                GeneralToken::Mode(i) => mode = *i,
                GeneralToken::LetterboxInBreaks(b) => letterbox_in_breaks = *b,
                GeneralToken::UseSkinSprites(b) => use_skin_sprites = *b,
                GeneralToken::OverlayPosition(s) => {
                    overlay_position = OverlayPosition::from(s.as_str())
                }
                GeneralToken::SkinPreference(s) => skin_preference = s.clone(),
                GeneralToken::EpilepsyWarning(b) => epilepsy_warning = *b,
                GeneralToken::CountdownOffset(i) => countdown_offset = *i,
                GeneralToken::SpecialStyle(b) => special_style = *b,
                GeneralToken::WidescreenStoryboard(b) => widescreen_storyboard = *b,
                GeneralToken::SamplesMatchSpeed(b) => samples_match_speed = *b,
                _ => {}
            }
        }

        General {
            audio_file,
            audio_lead_in,
            preview_time,
            countdown,
            sample_set,
            stack_leniency,
            mode,
            letterbox_in_breaks,
            use_skin_sprites,
            overlay_position,
            skin_preference,
            epilepsy_warning,
            countdown_offset,
            special_style,
            widescreen_storyboard,
            samples_match_speed,
        }
    }
}

impl std::fmt::Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[General]\n")?;
        write!(f, "AudioFilename: {}\n", self.audio_file.to_str().unwrap())?;
        write!(f, "AudioLeadIn: {}\n", self.audio_lead_in)?;
        write!(f, "PreviewTime: {}\n", self.preview_time)?;
        write!(f, "Countdown: {}\n", self.countdown)?;
        write!(f, "SampleSet: {}\n", self.sample_set)?;
        write!(f, "StackLeniency: {}\n", self.stack_leniency)?;
        write!(f, "Mode: {}\n", self.mode)?;
        write!(f, "LetterboxInBreaks: {}\n", self.letterbox_in_breaks)?;
        write!(f, "UseSkinSprites: {}\n", self.use_skin_sprites as u8)?;
        write!(f, "OverlayPosition: {}\n", self.overlay_position)?;
        write!(f, "SkinPreference: {}\n", self.skin_preference)?;
        write!(f, "EpilepsyWarning: {}\n", self.epilepsy_warning as u8)?;
        write!(f, "CountdownOffset: {}\n", self.countdown_offset)?;
        write!(f, "SpecialStyle: {}\n", self.special_style as u8)?;
        write!(
            f,
            "WidescreenStoryboard: {}\n",
            self.widescreen_storyboard as u8
        )?;
        write!(f, "SamplesMatchSpeed: {}\n", self.samples_match_speed as u8)
    }
}
