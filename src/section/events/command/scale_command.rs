#[derive(Debug)]
pub struct ScaleCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_scale: f32,
    end_scale: f32,
}

impl ScaleCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_time = if s.len() <= 4 {
            start_time
        } else {
            match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[2]),
                    ))
                }
            }
        };

        let end_scale: f32 = if s.len() == 4 {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        } else {
            match s[4].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        let start_scale: f32 = if s[s.len() - 2].is_empty() {
            end_scale
        } else {
            match s[s.len() - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        Ok(ScaleCommand {
            easing,
            start_time,
            end_time,
            start_scale,
            end_scale,
        })
    }
}

impl std::fmt::Display for ScaleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "S,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_scale, self.end_scale
        )
    }
}
