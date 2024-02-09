use crate::Beatmap;
#[derive(Debug)]
pub struct Difficulty {
    hp_drain_rate: f32,
    circle_size: f32,
    overall_difficulty: f32,
    approach_rate: f32,
    slider_multiplier: f32,
    slider_tick_rate: f32,
}

impl Difficulty {
    pub fn change_hp_drain_rate(&mut self, rate: f32) {
        self.hp_drain_rate = rate;
    }

    pub fn change_circle_size(&mut self, size: f32) {
        self.circle_size = size;
    }

    pub fn change_overall_difficulty(&mut self, difficulty: f32) {
        self.overall_difficulty = difficulty;
    }

    pub fn change_approach_rate(&mut self, rate: f32) {
        self.approach_rate = rate;
    }

    pub fn change_slider_multiplier(&mut self, multiplier: f32) {
        self.slider_multiplier = multiplier;
    }

    pub fn change_slider_tick_rate(&mut self, rate: f32) {
        self.slider_tick_rate = rate;
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Difficulty]\n")?;
        write!(f, "HPDrainRate: {}\n", self.hp_drain_rate)?;
        write!(f, "CircleSize: {}\n", self.circle_size)?;
        write!(f, "OverallDifficulty: {}\n", self.overall_difficulty)?;
        write!(f, "ApproachRate: {}\n", self.approach_rate)?;
        write!(f, "SliderMultiplier: {}\n", self.slider_multiplier)?;
        write!(f, "SliderTickRate: {}", self.slider_tick_rate)
    }
}

impl Beatmap {
    pub fn change_hp_drain_rate(&mut self, rate: f32) {
        self.difficulty.change_hp_drain_rate(rate);
    }

    pub fn change_circle_size(&mut self, size: f32) {
        self.difficulty.change_circle_size(size);
    }

    pub fn change_overall_difficulty(&mut self, difficulty: f32) {
        self.difficulty.change_overall_difficulty(difficulty);
    }

    pub fn change_approach_rate(&mut self, rate: f32) {
        self.difficulty.change_approach_rate(rate);
    }

    pub fn change_slider_multiplier(&mut self, multiplier: f32) {
        self.difficulty.change_slider_multiplier(multiplier);
    }

    pub fn change_slider_tick_rate(&mut self, rate: f32) {
        self.difficulty.change_slider_tick_rate(rate);
    }

    pub fn get_hp_drain_rate(&self) -> f32 {
        self.difficulty.hp_drain_rate
    }

    pub fn get_circle_size(&self) -> f32 {
        self.difficulty.circle_size
    }

    pub fn get_overall_difficulty(&self) -> f32 {
        self.difficulty.overall_difficulty
    }

    pub fn get_approach_rate(&self) -> f32 {
        self.difficulty.approach_rate
    }

    pub fn get_slider_multiplier(&self) -> f32 {
        self.difficulty.slider_multiplier
    }

    pub fn get_slider_tick_rate(&self) -> f32 {
        self.difficulty.slider_tick_rate
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty {
            hp_drain_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            approach_rate: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
        }
    }
}

impl Difficulty {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "HPDrainRate" => self.hp_drain_rate = value.parse().unwrap(),
            "CircleSize" => self.circle_size = value.parse().unwrap(),
            "OverallDifficulty" => self.overall_difficulty = value.parse().unwrap(),
            "ApproachRate" => self.approach_rate = value.parse().unwrap(),
            "SliderMultiplier" => self.slider_multiplier = value.parse().unwrap(),
            "SliderTickRate" => self.slider_tick_rate = value.parse().unwrap(),
            _ => {}
        }
    }
}
