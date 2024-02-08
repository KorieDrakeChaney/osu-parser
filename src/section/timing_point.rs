#[derive(Debug)]
pub struct TimingPoint {
    time: i32,
    beat_length: f32,
    meter: i32,
    sample_set: i32,
    sample_index: i32,
    volume: i32,
    uninherited: bool,
    effects: i32,
}

impl TimingPoint {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        Ok(TimingPoint {
            time: {
                if let Ok(n) = s[0].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            beat_length: {
                if let Ok(n) = s[1].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            meter: {
                if let Ok(n) = s[2].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            sample_set: {
                if let Ok(n) = s[3].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            sample_index: {
                if let Ok(n) = s[4].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            volume: {
                if let Ok(n) = s[5].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
            uninherited: s[6] == "1",
            effects: {
                if let Ok(n) = s[7].parse() {
                    n
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Invalid TimingPoint token",
                    ));
                }
            },
        })
    }
}

impl std::fmt::Display for TimingPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{}\n",
            self.time,
            self.beat_length,
            self.meter,
            self.sample_set,
            self.sample_index,
            self.volume,
            if self.uninherited { 1 } else { 0 },
            self.effects
        )
    }
}
