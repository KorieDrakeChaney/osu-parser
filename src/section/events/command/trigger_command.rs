#[derive(Debug)]
pub struct TriggerCommand {
    trigger_type: Trigger,
    start_time: i32,
    end_time: i32,
    group: Option<String>,
}

impl std::fmt::Display for TriggerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "T,{},{},{}",
            self.trigger_type, self.start_time, self.end_time
        );

        if let Some(group) = &self.group {
            display_string.push_str(&format!(",{}", group));
        }

        write!(f, "{}\n", display_string)
    }
}

impl TriggerCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let trigger_type = Trigger::from(s[0]);

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Trigger Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Trigger Command token: {}", s[2]),
                ))
            }
        };

        let group = if s.len() > 3 {
            Some(s[3].to_string())
        } else {
            None
        };

        Ok(TriggerCommand {
            trigger_type,
            start_time,
            end_time,
            group,
        })
    }
}

#[derive(Debug)]
pub enum Trigger {
    HitSoundClap,
    HitSoundFinish,
    HitSoundWhistle,
    Passing,
    Failing,
}

impl From<&str> for Trigger {
    fn from(s: &str) -> Self {
        match s {
            "HitSoundClap" => Trigger::HitSoundClap,
            "HitSoundFinish" => Trigger::HitSoundFinish,
            "HitSoundWhistle" => Trigger::HitSoundWhistle,
            "Passing" => Trigger::Passing,
            "Failing" => Trigger::Failing,
            _ => Trigger::HitSoundClap,
        }
    }
}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Trigger::HitSoundClap => write!(f, "HitSoundClap"),
            Trigger::HitSoundFinish => write!(f, "HitSoundFinish"),
            Trigger::HitSoundWhistle => write!(f, "HitSoundWhistle"),
            Trigger::Passing => write!(f, "Passing"),
            Trigger::Failing => write!(f, "Failing"),
        }
    }
}
