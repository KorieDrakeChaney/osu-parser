#[derive(Debug)]
pub enum Effect {
    Additive,
    HorizontalFlip,
    VerticalFlip,
    OriginAtCenter,
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Effect::Additive => write!(f, "A"),
            Effect::HorizontalFlip => write!(f, "H"),
            Effect::VerticalFlip => write!(f, "V"),
            Effect::OriginAtCenter => write!(f, "R"),
        }
    }
}

impl From<&str> for Effect {
    fn from(s: &str) -> Self {
        match s {
            "A" => Effect::Additive,
            "H" => Effect::HorizontalFlip,
            "V" => Effect::VerticalFlip,
            "R" => Effect::OriginAtCenter,
            _ => Effect::Additive,
        }
    }
}

#[derive(Debug)]
pub struct ParameterCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    parameter_type: Effect,
}

impl std::fmt::Display for ParameterCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "P,{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.parameter_type
        )
    }
}

impl ParameterCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 4", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[2]),
                ))
            }
        };

        let parameter_type = Effect::from(s[3]);

        Ok(ParameterCommand {
            easing,
            start_time,
            end_time,
            parameter_type,
        })
    }
}
