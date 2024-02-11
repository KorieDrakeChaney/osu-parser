#[derive(Debug)]
pub struct FadeCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_opacity: f32,
    end_opacity: f32,
}

impl FadeCommand {
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

        let has_end_opacity = (s.len() == 4 && s[s.len() - 2].is_empty()) || s.len() == 5;

        let end_opacity = match s[s.len() - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[s.len() - 1]),
                ))
            }
        };

        let start_opacity = if s[s.len() - 2].is_empty() || !has_end_opacity {
            end_opacity
        } else {
            match s[s.len() - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[s.len() - 2]),
                    ))
                }
            }
        };

        let end_time = if s.len() == 4 && has_end_opacity {
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

        Ok(FadeCommand {
            easing,
            start_time,
            end_time,
            start_opacity,
            end_opacity,
        })
    }
}

impl std::fmt::Display for FadeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "F,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_opacity, self.end_opacity
        )
    }
}
