#[derive(Debug)]
pub struct MoveYCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_y: i32,
    end_y: i32,
}

impl std::fmt::Display for MoveYCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MY,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_y, self.end_y
        )
    }
}

impl MoveYCommand {
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

        let end_y = match s[s.len() - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveX Command token: {}", s[4]),
                ))
            }
        };

        let start_y = if s[s.len() - 2].is_empty() {
            end_y
        } else {
            match s[s.len() - 2].parse() {
                Ok(y) => y,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid MoveX Command token: {}", s[3]),
                    ))
                }
            }
        };

        let end_time = if s.len() <= 4 {
            start_time
        } else {
            match s[2].parse() {
                Ok(y) => y,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid MoveX Command token: {}", s[2]),
                    ))
                }
            }
        };

        Ok(MoveYCommand {
            easing,
            start_time,
            end_time,
            start_y,
            end_y,
        })
    }
}
