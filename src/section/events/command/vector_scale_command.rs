#[derive(Debug)]
pub struct VectorScaleCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_scale_x: f32,
    start_scale_y: f32,
    end_scale_x: f32,
    end_scale_y: f32,
}

impl std::fmt::Display for VectorScaleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "V,{},{},{},{},{},{},{}\n",
            self.easing,
            self.start_time,
            self.end_time,
            self.start_scale_x,
            self.end_scale_x,
            self.start_scale_y,
            self.end_scale_y
        )
    }
}

impl VectorScaleCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let start_scale_x;
        let start_scale_y;
        let end_time;

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid VectorScale Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid VectorScale Command token: {}", s[1]),
                ))
            }
        };

        let end_index = s.len() - 1;

        let end_scale_y = match s[end_index].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid VectorScale Command token: {}", s[end_index]),
                ))
            }
        };

        let end_scale_x = match s[end_index - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid VectorScale Command token: {}", s[end_index - 1]),
                ))
            }
        };

        if s[end_index - 2].is_empty() && end_index - 3 == 1 {
            start_scale_x = end_scale_x;
            start_scale_y = end_scale_y;
            end_time = start_time;
        } else if s[end_index - 2].is_empty() {
            start_scale_y = end_scale_y;
            end_time = start_time;
            start_scale_x = match s[end_index - 3].parse() {
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
            start_scale_y = match s[end_index - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 2]),
                    ))
                }
            };
            start_scale_x = match s[end_index - 3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 3]),
                    ))
                }
            };
        }

        Ok(VectorScaleCommand {
            easing,
            start_time,
            end_time,
            start_scale_x,
            end_scale_x,
            start_scale_y,
            end_scale_y,
        })
    }
}
