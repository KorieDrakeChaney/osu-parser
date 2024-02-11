#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub audio_file: Option<String>,
    /// Milliseconds of silence before the audio starts playing
    pub audio_lead_in: Option<i32>,
    /// Time in milliseconds when the audio preview should start
    pub preview_time: Option<i32>,
    /// Speed of the coundown before the first hit object
    pub countdown: Option<i32>,
    /// Sample set that will be used if timing points do not override it
    pub sample_set: Option<SampleSet>,
    /// Multiplier for teh threshold in the time where hit objects placed close together stack
    pub stack_leniency: Option<f32>,
    /// Game mode
    pub mode: Option<i32>,
    /// Whether or not breaks have a letterboxing effect
    pub letterbox_in_breaks: Option<bool>,
    /// Whether or not the storyboard can use the user's skin images
    pub use_skin_sprites: Option<bool>,
    ///  Draw order of hit circle overlays compared to hit numbers
    pub overlay_position: Option<OverlayPosition>,
    /// Preferred skin to use during gameplay
    pub skin_preference: Option<String>,
    /// Epilepsy warning
    pub epilepsy_warning: Option<bool>,
    /// Time in beats that the countdown starts before the first hit object
    pub countdown_offset: Option<i32>,
    /// Where or not the "N+1" style key layout is used for osu!mania
    pub special_style: Option<bool>,
    /// Where or not the storyboard allows widescreen viewing
    pub widescreen_storyboard: Option<bool>,
    /// Where or not sound samples will change rate when playing with speed-changing mods
    pub samples_match_playback_rate: Option<bool>,

    /// Deprecated
    pub audio_hash: Option<String>,
    /// Deprecated
    pub story_fire_in_front: Option<bool>,
    /// Deprecated
    pub always_show_playfield: Option<bool>,
}

impl Default for General {
    fn default() -> Self {
        General {
            audio_file: None,
            audio_lead_in: None,
            preview_time: None,
            countdown: None,
            sample_set: None,
            stack_leniency: None,
            mode: None,
            letterbox_in_breaks: None,
            use_skin_sprites: None,
            overlay_position: None,
            skin_preference: None,
            epilepsy_warning: None,
            countdown_offset: None,
            special_style: None,
            widescreen_storyboard: None,
            samples_match_playback_rate: None,

            audio_hash: None,
            story_fire_in_front: None,
            always_show_playfield: None,
        }
    }
}

impl std::fmt::Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = String::from("[General]\n");
        if let Some(audio_file) = &self.audio_file {
            display_string += &format!("AudioFilename: {}\n", audio_file);
        }
        if let Some(audio_lead_in) = self.audio_lead_in {
            display_string += &format!("AudioLeadIn: {}\n", audio_lead_in);
        }
        if let Some(audio_hash) = &self.audio_hash {
            display_string += &format!("AudioHash: {}\n", audio_hash);
        }
        if let Some(preview_time) = self.preview_time {
            display_string += &format!("PreviewTime: {}\n", preview_time);
        }
        if let Some(countdown) = self.countdown {
            display_string += &format!("Countdown: {}\n", countdown);
        }
        if let Some(sample_set) = self.sample_set {
            display_string += &format!("SampleSet: {}\n", sample_set);
        }
        if let Some(stack_leniency) = self.stack_leniency {
            display_string += &format!("StackLeniency: {}\n", stack_leniency);
        }
        if let Some(mode) = self.mode {
            display_string += &format!("Mode: {}\n", mode);
        }
        if let Some(letterbox_in_breaks) = self.letterbox_in_breaks {
            display_string += &format!("LetterboxInBreaks: {}\n", letterbox_in_breaks as u8);
        }
        if let Some(story_fire_in_front) = self.story_fire_in_front {
            display_string += &format!("StoryFireInFront: {}\n", story_fire_in_front as u8);
        }
        if let Some(use_skin_sprites) = self.use_skin_sprites {
            display_string += &format!("UseSkinSprites: {}\n", use_skin_sprites as u8);
        }
        if let Some(always_show_playfield) = self.always_show_playfield {
            display_string += &format!("AlwaysShowPlayfield: {}\n", always_show_playfield as u8);
        }
        if let Some(overlay_position) = self.overlay_position {
            display_string += &format!("OverlayPosition: {}\n", overlay_position);
        }
        if let Some(skin_preference) = &self.skin_preference {
            display_string += &format!("SkinPreference: {}\n", skin_preference);
        }
        if let Some(epilepsy_warning) = self.epilepsy_warning {
            display_string += &format!("EpilepsyWarning: {}\n", epilepsy_warning as u8);
        }
        if let Some(countdown_offset) = self.countdown_offset {
            display_string += &format!("CountdownOffset: {}\n", countdown_offset);
        }
        if let Some(special_style) = self.special_style {
            display_string += &format!("SpecialStyle: {}\n", special_style as u8);
        }
        if let Some(widescreen_storyboard) = self.widescreen_storyboard {
            display_string += &format!("WidescreenStoryboard: {}\n", widescreen_storyboard as u8);
        }
        if let Some(samples_match_playback_rate) = self.samples_match_playback_rate {
            display_string += &format!(
                "SamplesMatchPlaybackRate: {}\n",
                samples_match_playback_rate as i32
            );
        }

        write!(f, "{}", display_string)
    }
}

impl General {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "AudioFilename" => self.audio_file = Some(value.to_string()),
            "AudioLeadIn" => self.audio_lead_in = Some(value.parse().unwrap()),
            "PreviewTime" => self.preview_time = Some(value.parse().unwrap()),
            "Countdown" => self.countdown = Some(value.parse().unwrap()),
            "SampleSet" => self.sample_set = Some(SampleSet::from(value)),
            "StackLeniency" => self.stack_leniency = Some(value.parse().unwrap()),
            "Mode" => self.mode = Some(value.parse().unwrap()),
            "LetterboxInBreaks" => self.letterbox_in_breaks = Some(value != "0"),
            "UseSkinSprites" => self.use_skin_sprites = Some(value != "0"),
            "OverlayPosition" => self.overlay_position = Some(OverlayPosition::from(value)),
            "SkinPreference" => self.skin_preference = Some(value.to_string()),
            "EpilepsyWarning" => self.epilepsy_warning = Some(value != "0"),
            "CountdownOffset" => self.countdown_offset = Some(value.parse().unwrap()),
            "SpecialStyle" => self.special_style = Some(value != "0"),
            "WidescreenStoryboard" => self.widescreen_storyboard = Some(value != "0"),
            "SamplesMatchPlaybackRate" => self.samples_match_playback_rate = Some(value != "0"),

            "AudioHash" => self.audio_hash = Some(value.to_string()),
            "StoryFireInFront" => self.story_fire_in_front = Some(value != "0"),
            "AlwaysShowPlayfield" => self.always_show_playfield = Some(value != "0"),
            _ => {}
        }
    }
}
