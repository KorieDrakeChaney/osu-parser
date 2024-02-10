#[derive(Debug)]
pub struct MoveCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
}

impl std::fmt::Display for MoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "M,{},{},{},{},{},{},{}\n",
            self.easing,
            self.start_time,
            self.end_time,
            self.start_x,
            self.start_y,
            self.end_x,
            self.end_y
        )
    }
}

impl MoveCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let start_x;
        let start_y;
        let end_time;

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

        let end_index = s.len() - 1;

        let end_x = match s[end_index - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[end_index - 2]),
                ))
            }
        };

        let end_y = match s[end_index].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[end_index]),
                ))
            }
        };

        if s[end_index - 2].is_empty() && end_index - 3 == 1 {
            start_x = end_x;
            start_y = end_y;
            end_time = start_time;
        } else if s[end_index - 2].is_empty() {
            start_y = end_y;
            end_time = start_time;
            start_x = match s[end_index - 3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 4]),
                    ))
                }
            }
        } else {
            end_time = match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 1]),
                    ))
                }
            };
            start_y = match s[end_index - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 2]),
                    ))
                }
            };
            start_x = match s[end_index - 3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 3]),
                    ))
                }
            };
        }

        Ok(MoveCommand {
            easing,
            start_time,
            end_time,
            start_x,
            start_y,
            end_x,
            end_y,
        })
    }
}
