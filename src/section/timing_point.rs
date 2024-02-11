#[derive(Debug)]
pub struct TimingPoint {
    time: i32,
    beat_length: Option<f32>,
    meter: Option<i32>,
    sample_set: Option<i32>,
    sample_index: Option<i32>,
    volume: Option<i32>,
    uninherited: Option<bool>,
    effects: Option<i32>,
}

impl TimingPoint {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        Ok(TimingPoint {
            time: {
                if let Ok(n) = s[0].parse() {
                    n
                } else {
                    0
                }
            },
            beat_length: {
                if s.len() > 1 {
                    if let Ok(n) = s[1].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
            meter: {
                if s.len() > 2 {
                    if let Ok(n) = s[2].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
            sample_set: {
                if s.len() > 3 {
                    if let Ok(n) = s[3].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
            sample_index: {
                if s.len() > 4 {
                    if let Ok(n) = s[4].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
            volume: {
                if s.len() > 5 {
                    if let Ok(n) = s[5].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
            uninherited: {
                if s.len() > 6 {
                    Some(s[6] == "1")
                } else {
                    None
                }
            },
            effects: {
                if s.len() > 7 {
                    if let Ok(n) = s[7].parse() {
                        Some(n)
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "Invalid TimingPoint token",
                        ));
                    }
                } else {
                    None
                }
            },
        })
    }
}

impl std::fmt::Display for TimingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!("{}", self.time);
        if let Some(beat_length) = self.beat_length {
            display_string.push_str(&format!(",{}", beat_length));
        }
        if let Some(meter) = self.meter {
            display_string.push_str(&format!(",{}", meter));
        }
        if let Some(sample_set) = self.sample_set {
            display_string.push_str(&format!(",{}", sample_set));
        }
        if let Some(sample_index) = self.sample_index {
            display_string.push_str(&format!(",{}", sample_index));
        }
        if let Some(volume) = self.volume {
            display_string.push_str(&format!(",{}", volume));
        }
        if let Some(uninherited) = self.uninherited {
            display_string.push_str(&format!(",{}", if uninherited { 1 } else { 0 }));
        }
        if let Some(effects) = self.effects {
            display_string.push_str(&format!(",{}", effects));
        }
        write!(f, "{}\n", display_string)
    }
}
