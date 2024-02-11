#[derive(Debug)]
pub struct MoveXCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_x: i32,
    end_x: i32,
}

impl std::fmt::Display for MoveXCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MX,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_x, self.end_x
        )
    }
}

impl MoveXCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveX Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveX Command token: {}", s[1]),
                ))
            }
        };

        let has_end_position = (s.len() == 4 && s[s.len() - 2].is_empty()) || s.len() == 5;

        let end_x = match s[s.len() - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveX Command token: {}", s[4]),
                ))
            }
        };

        let start_x = if s[s.len() - 2].is_empty() || !has_end_position {
            end_x
        } else {
            match s[s.len() - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid MoveX Command token: {}", s[3]),
                    ))
                }
            }
        };

        let end_time = if s.len() == 4 && has_end_position {
            start_time
        } else {
            match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid MoveX Command token: {}", s[2]),
                    ))
                }
            }
        };

        Ok(MoveXCommand {
            easing,
            start_time,
            end_time,
            start_x,
            end_x,
        })
    }
}
