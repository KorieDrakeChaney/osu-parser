use crate::{token::DifficultyToken, Beatmap};
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

impl From<&Vec<DifficultyToken>> for Difficulty {
    fn from(tokens: &Vec<DifficultyToken>) -> Self {
        let mut hp_drain_rate = 5.0;
        let mut circle_size = 5.0;
        let mut overall_difficulty = 5.0;
        let mut approach_rate = 5.0;
        let mut slider_multiplier = 1.4;
        let mut slider_tick_rate = 1.0;

        for token in tokens {
            match token {
                DifficultyToken::HPDrainRate(rate) => hp_drain_rate = *rate,
                DifficultyToken::CircleSize(size) => circle_size = *size,
                DifficultyToken::OverallDifficulty(difficulty) => overall_difficulty = *difficulty,
                DifficultyToken::ApproachRate(rate) => approach_rate = *rate,
                DifficultyToken::SliderMultiplier(multiplier) => slider_multiplier = *multiplier,
                DifficultyToken::SliderTickRate(rate) => slider_tick_rate = *rate,
            }
        }

        Difficulty {
            hp_drain_rate,
            circle_size,
            overall_difficulty,
            approach_rate,
            slider_multiplier,
            slider_tick_rate,
        }
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
}
