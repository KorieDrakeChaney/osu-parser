use crate::Beatmap;

#[derive(Debug)]
pub struct Difficulty {
    hp_drain_rate: Option<f32>,
    circle_size: Option<f32>,
    overall_difficulty: Option<f32>,
    approach_rate: Option<f32>,
    slider_multiplier: Option<f32>,
    slider_tick_rate: Option<f32>,
}

impl Difficulty {
    pub fn change_hp_drain_rate(&mut self, rate: f32) {
        self.hp_drain_rate = Some(rate);
    }

    pub fn change_circle_size(&mut self, size: f32) {
        self.circle_size = Some(size);
    }

    pub fn change_overall_difficulty(&mut self, difficulty: f32) {
        self.overall_difficulty = Some(difficulty);
    }

    pub fn change_approach_rate(&mut self, rate: f32) {
        self.approach_rate = Some(rate);
    }

    pub fn change_slider_multiplier(&mut self, multiplier: f32) {
        self.slider_multiplier = Some(multiplier);
    }

    pub fn change_slider_tick_rate(&mut self, rate: f32) {
        self.slider_tick_rate = Some(rate);
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = String::from("[Difficulty]\n");

        if let Some(hp_drain_rate) = &self.hp_drain_rate {
            display_string.push_str(&format!("HPDrainRate: {}\n", hp_drain_rate));
        }

        if let Some(circle_size) = &self.circle_size {
            display_string.push_str(&format!("CircleSize: {}\n", circle_size));
        }

        if let Some(overall_difficulty) = &self.overall_difficulty {
            display_string.push_str(&format!("OverallDifficulty: {}\n", overall_difficulty));
        }

        if let Some(approach_rate) = &self.approach_rate {
            display_string.push_str(&format!("ApproachRate: {}\n", approach_rate));
        }

        if let Some(slider_multiplier) = &self.slider_multiplier {
            display_string.push_str(&format!("SliderMultiplier: {}\n", slider_multiplier));
        }

        if let Some(slider_tick_rate) = &self.slider_tick_rate {
            display_string.push_str(&format!("SliderTickRate: {}\n", slider_tick_rate));
        }

        write!(f, "{}", display_string)
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

    pub fn get_hp_drain_rate(&self) -> Option<f32> {
        self.difficulty.hp_drain_rate
    }

    pub fn get_circle_size(&self) -> Option<f32> {
        self.difficulty.circle_size
    }

    pub fn get_overall_difficulty(&self) -> Option<f32> {
        self.difficulty.overall_difficulty
    }

    pub fn get_approach_rate(&self) -> Option<f32> {
        self.difficulty.approach_rate
    }

    pub fn get_slider_multiplier(&self) -> Option<f32> {
        self.difficulty.slider_multiplier
    }

    pub fn get_slider_tick_rate(&self) -> Option<f32> {
        self.difficulty.slider_tick_rate
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty {
            hp_drain_rate: None,
            circle_size: None,
            overall_difficulty: None,
            approach_rate: None,
            slider_multiplier: None,
            slider_tick_rate: None,
        }
    }
}

impl Difficulty {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];
        match parts[0] {
            "HPDrainRate" => {
                if let Ok(n) = value.parse() {
                    self.hp_drain_rate = Some(n);
                }
            }
            "CircleSize" => {
                if let Ok(n) = value.parse() {
                    self.circle_size = Some(n);
                }
            }
            "OverallDifficulty" => {
                if let Ok(n) = value.parse() {
                    self.overall_difficulty = Some(n);
                }
            }
            "ApproachRate" => {
                if let Ok(n) = value.parse() {
                    self.approach_rate = Some(n);
                }
            }
            "SliderMultiplier" => {
                if let Ok(n) = value.parse() {
                    self.slider_multiplier = Some(n);
                }
            }
            "SliderTickRate" => {
                if let Ok(n) = value.parse() {
                    self.slider_tick_rate = Some(n);
                }
            }

            _ => {}
        }
    }
}
