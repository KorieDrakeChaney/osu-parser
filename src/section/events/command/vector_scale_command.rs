#[derive(Debug)]
pub struct VectorScaleCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
}

impl std::fmt::Display for VectorScaleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "V,{},{},{},{},{},{},{}\n",
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

impl VectorScaleCommand {
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

        let has_end_xy = s.len() == 7 || (s.len() == 5 && s[s.len() - 3].is_empty());

        let end_y = match s[s.len() - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[4]),
                ))
            }
        };

        let end_x = match s[s.len() - 2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[3]),
                ))
            }
        };

        let start_y = if s[s.len() - 3].is_empty() || !has_end_xy {
            end_y
        } else {
            match s[s.len() - 3].parse() {
                Ok(y) => y,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[2]),
                    ))
                }
            }
        };

        let start_x = if s[s.len() - 3].is_empty() || !has_end_xy {
            end_x
        } else {
            match s[s.len() - 4].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[1]),
                    ))
                }
            }
        };

        let end_time = if s.len() == 5 && has_end_xy {
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

        Ok(VectorScaleCommand {
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
