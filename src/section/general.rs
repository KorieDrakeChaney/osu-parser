use std::ffi::OsString;

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
    pub samples_match_playback_rate: bool,
}

impl Default for General {
    fn default() -> Self {
        General {
            audio_file: OsString::new(),
            audio_lead_in: 0,
            preview_time: 0,
            countdown: 0,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: 0,
            letterbox_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
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
        write!(f, "LetterboxInBreaks: {}\n", self.letterbox_in_breaks as u8)?;
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
        write!(
            f,
            "SamplesMatchSpeed: {}\n",
            self.samples_match_playback_rate as u8
        )
    }
}

impl General {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "AudioFilename" => {
                self.audio_file = OsString::from(value);
            }
            "AudioLeadIn" => {
                if let Ok(n) = value.parse() {
                    self.audio_lead_in = n;
                }
            }
            "PreviewTime" => {
                if let Ok(n) = value.parse() {
                    self.preview_time = n;
                }
            }
            "Countdown" => {
                if let Ok(n) = value.parse() {
                    self.countdown = n;
                }
            }
            "SampleSet" => {
                self.sample_set = SampleSet::from(value);
            }
            "StackLeniency" => {
                if let Ok(n) = value.parse() {
                    self.stack_leniency = n;
                }
            }
            "Mode" => {
                if let Ok(n) = value.parse() {
                    self.mode = n;
                }
            }
            "LetterboxInBreaks" => {
                self.letterbox_in_breaks = value != "0";
            }
            "UseSkinSprites" => {
                self.use_skin_sprites = value != "0";
            }
            "OverlayPosition" => {
                self.overlay_position = OverlayPosition::from(value);
            }
            "SkinPreference" => {
                self.skin_preference = value.to_string();
            }
            "EpilepsyWarning" => {
                self.epilepsy_warning = value != "0";
            }
            "CountdownOffset" => {
                if let Ok(n) = value.parse() {
                    self.countdown_offset = n;
                }
            }
            "SpecialStyle" => {
                self.special_style = value != "0";
            }
            "WidescreenStoryboard" => {
                self.widescreen_storyboard = value != "0";
            }
            "SamplesMatchPlaybackRate" => {
                self.samples_match_playback_rate = value != "0";
            }
            _ => {}
        }
    }
}
