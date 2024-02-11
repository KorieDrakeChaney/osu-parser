use crate::Beatmap;

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

impl General {
    pub fn get_audio_file(&self) -> Option<String> {
        self.audio_file.clone()
    }

    pub fn get_audio_lead_in(&self) -> Option<i32> {
        self.audio_lead_in
    }

    pub fn get_preview_time(&self) -> Option<i32> {
        self.preview_time
    }

    pub fn get_countdown(&self) -> Option<i32> {
        self.countdown
    }

    pub fn get_sample_set(&self) -> Option<SampleSet> {
        self.sample_set
    }

    pub fn get_stack_leniency(&self) -> Option<f32> {
        self.stack_leniency
    }

    pub fn get_mode(&self) -> Option<i32> {
        self.mode
    }

    pub fn get_letterbox_in_breaks(&self) -> Option<bool> {
        self.letterbox_in_breaks
    }

    pub fn get_use_skin_sprites(&self) -> Option<bool> {
        self.use_skin_sprites
    }

    pub fn get_overlay_position(&self) -> Option<OverlayPosition> {
        self.overlay_position
    }

    pub fn get_skin_preference(&self) -> Option<String> {
        self.skin_preference.clone()
    }

    pub fn get_epilepsy_warning(&self) -> Option<bool> {
        self.epilepsy_warning
    }

    pub fn get_countdown_offset(&self) -> Option<i32> {
        self.countdown_offset
    }

    pub fn get_special_style(&self) -> Option<bool> {
        self.special_style
    }

    pub fn get_widescreen_storyboard(&self) -> Option<bool> {
        self.widescreen_storyboard
    }

    pub fn get_samples_match_playback_rate(&self) -> Option<bool> {
        self.samples_match_playback_rate
    }

    pub fn get_audio_hash(&self) -> Option<String> {
        self.audio_hash.clone()
    }

    pub fn get_story_fire_in_front(&self) -> Option<bool> {
        self.story_fire_in_front
    }

    pub fn get_always_show_playfield(&self) -> Option<bool> {
        self.always_show_playfield
    }

    pub fn change_audio_file(&mut self, audio_file: &str) {
        self.audio_file = Some(audio_file.to_string());
    }

    pub fn change_audio_lead_in(&mut self, audio_lead_in: i32) {
        self.audio_lead_in = Some(audio_lead_in);
    }

    pub fn change_preview_time(&mut self, preview_time: i32) {
        self.preview_time = Some(preview_time);
    }

    pub fn change_countdown(&mut self, countdown: i32) {
        self.countdown = Some(countdown);
    }

    pub fn change_sample_set(&mut self, sample_set: SampleSet) {
        self.sample_set = Some(sample_set);
    }

    pub fn change_stack_leniency(&mut self, stack_leniency: f32) {
        self.stack_leniency = Some(stack_leniency);
    }

    pub fn change_mode(&mut self, mode: i32) {
        self.mode = Some(mode);
    }

    pub fn change_letterbox_in_breaks(&mut self, letterbox_in_breaks: bool) {
        self.letterbox_in_breaks = Some(letterbox_in_breaks);
    }

    pub fn change_use_skin_sprites(&mut self, use_skin_sprites: bool) {
        self.use_skin_sprites = Some(use_skin_sprites);
    }

    pub fn change_overlay_position(&mut self, overlay_position: OverlayPosition) {
        self.overlay_position = Some(overlay_position);
    }

    pub fn change_skin_preference(&mut self, skin_preference: &str) {
        self.skin_preference = Some(skin_preference.to_string());
    }

    pub fn change_epilepsy_warning(&mut self, epilepsy_warning: bool) {
        self.epilepsy_warning = Some(epilepsy_warning);
    }

    pub fn change_countdown_offset(&mut self, countdown_offset: i32) {
        self.countdown_offset = Some(countdown_offset);
    }

    pub fn change_special_style(&mut self, special_style: bool) {
        self.special_style = Some(special_style);
    }

    pub fn change_widescreen_storyboard(&mut self, widescreen_storyboard: bool) {
        self.widescreen_storyboard = Some(widescreen_storyboard);
    }

    pub fn change_samples_match_playback_rate(&mut self, samples_match_playback_rate: bool) {
        self.samples_match_playback_rate = Some(samples_match_playback_rate);
    }

    pub fn change_audio_hash(&mut self, audio_hash: &str) {
        self.audio_hash = Some(audio_hash.to_string());
    }

    pub fn change_story_fire_in_front(&mut self, story_fire_in_front: bool) {
        self.story_fire_in_front = Some(story_fire_in_front);
    }

    pub fn change_always_show_playfield(&mut self, always_show_playfield: bool) {
        self.always_show_playfield = Some(always_show_playfield);
    }
}

impl Beatmap {
    pub fn get_general_audio_file(&self) -> Option<String> {
        self.general.get_audio_file()
    }

    pub fn get_general_audio_lead_in(&self) -> Option<i32> {
        self.general.get_audio_lead_in()
    }

    pub fn get_general_preview_time(&self) -> Option<i32> {
        self.general.get_preview_time()
    }

    pub fn get_general_countdown(&self) -> Option<i32> {
        self.general.get_countdown()
    }

    pub fn get_general_sample_set(&self) -> Option<SampleSet> {
        self.general.get_sample_set()
    }

    pub fn get_general_stack_leniency(&self) -> Option<f32> {
        self.general.get_stack_leniency()
    }

    pub fn get_general_mode(&self) -> Option<i32> {
        self.general.get_mode()
    }

    pub fn get_general_letterbox_in_breaks(&self) -> Option<bool> {
        self.general.get_letterbox_in_breaks()
    }

    pub fn get_general_use_skin_sprites(&self) -> Option<bool> {
        self.general.get_use_skin_sprites()
    }

    pub fn get_general_overlay_position(&self) -> Option<OverlayPosition> {
        self.general.get_overlay_position()
    }

    pub fn get_general_skin_preference(&self) -> Option<String> {
        self.general.get_skin_preference()
    }

    pub fn get_general_epilepsy_warning(&self) -> Option<bool> {
        self.general.get_epilepsy_warning()
    }

    pub fn get_general_countdown_offset(&self) -> Option<i32> {
        self.general.get_countdown_offset()
    }

    pub fn get_general_special_style(&self) -> Option<bool> {
        self.general.get_special_style()
    }

    pub fn get_general_widescreen_storyboard(&self) -> Option<bool> {
        self.general.get_widescreen_storyboard()
    }

    pub fn get_general_samples_match_playback_rate(&self) -> Option<bool> {
        self.general.get_samples_match_playback_rate()
    }

    pub fn get_general_audio_hash(&self) -> Option<String> {
        self.general.get_audio_hash()
    }

    pub fn get_general_story_fire_in_front(&self) -> Option<bool> {
        self.general.get_story_fire_in_front()
    }

    pub fn get_general_always_show_playfield(&self) -> Option<bool> {
        self.general.get_always_show_playfield()
    }

    pub fn change_general_audio_file(&mut self, audio_file: &str) {
        self.general.change_audio_file(audio_file);
    }

    pub fn change_general_audio_lead_in(&mut self, audio_lead_in: i32) {
        self.general.change_audio_lead_in(audio_lead_in);
    }

    pub fn change_general_preview_time(&mut self, preview_time: i32) {
        self.general.change_preview_time(preview_time);
    }

    pub fn change_general_countdown(&mut self, countdown: i32) {
        self.general.change_countdown(countdown);
    }

    pub fn change_general_sample_set(&mut self, sample_set: SampleSet) {
        self.general.change_sample_set(sample_set);
    }

    pub fn change_general_stack_leniency(&mut self, stack_leniency: f32) {
        self.general.change_stack_leniency(stack_leniency);
    }

    pub fn change_general_mode(&mut self, mode: i32) {
        self.general.change_mode(mode);
    }

    pub fn change_general_letterbox_in_breaks(&mut self, letterbox_in_breaks: bool) {
        self.general.change_letterbox_in_breaks(letterbox_in_breaks);
    }

    pub fn change_general_use_skin_sprites(&mut self, use_skin_sprites: bool) {
        self.general.change_use_skin_sprites(use_skin_sprites);
    }

    pub fn change_general_overlay_position(&mut self, overlay_position: OverlayPosition) {
        self.general.change_overlay_position(overlay_position);
    }

    pub fn change_general_skin_preference(&mut self, skin_preference: &str) {
        self.general.change_skin_preference(skin_preference);
    }

    pub fn change_general_epilepsy_warning(&mut self, epilepsy_warning: bool) {
        self.general.change_epilepsy_warning(epilepsy_warning);
    }

    pub fn change_general_countdown_offset(&mut self, countdown_offset: i32) {
        self.general.change_countdown_offset(countdown_offset);
    }

    pub fn change_general_special_style(&mut self, special_style: bool) {
        self.general.change_special_style(special_style);
    }

    pub fn change_general_widescreen_storyboard(&mut self, widescreen_storyboard: bool) {
        self.general
            .change_widescreen_storyboard(widescreen_storyboard);
    }

    pub fn change_general_samples_match_playback_rate(
        &mut self,
        samples_match_playback_rate: bool,
    ) {
        self.general
            .change_samples_match_playback_rate(samples_match_playback_rate);
    }

    pub fn change_general_audio_hash(&mut self, audio_hash: &str) {
        self.general.change_audio_hash(audio_hash);
    }

    pub fn change_general_story_fire_in_front(&mut self, story_fire_in_front: bool) {
        self.general.change_story_fire_in_front(story_fire_in_front);
    }

    pub fn change_general_always_show_playfield(&mut self, always_show_playfield: bool) {
        self.general
            .change_always_show_playfield(always_show_playfield);
    }
}
