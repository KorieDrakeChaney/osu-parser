#[derive(Debug)]
pub struct RotateCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_angle: f32,
    end_angle: f32,
}

impl std::fmt::Display for RotateCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "R,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_angle, self.end_angle
        )
    }
}

impl RotateCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Rotate Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Rotate Command token: {}", s[1]),
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

        let end_angle: f32 = match s[s.len() - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[3]),
                ))
            }
        };

        let start_angle: f32 = if s[s.len() - 2].is_empty() {
            end_angle
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

        Ok(RotateCommand {
            easing,
            start_time,
            end_time,
            start_angle,
            end_angle,
        })
    }
}
