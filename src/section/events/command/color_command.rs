use crate::utils::Color;

#[derive(Debug)]
pub struct ColorCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_color: Color,
    end_color: Color,
}

impl std::fmt::Display for ColorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "C,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_color, self.end_color
        )
    }
}

impl ColorCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[1]),
                ))
            }
        };

        let has_end_color = (s.len() == 6 && s[s.len() - 4].is_empty()) || s.len() == 9;

        let end_color = if let (Some(r), Some(g), Some(b)) = (
            s[s.len() - 3].parse().ok(),
            s[s.len() - 2].parse().ok(),
            s[s.len() - 1].parse().ok(),
        ) {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
        };

        let start_color = if s[s.len() - 4].is_empty() || !has_end_color {
            end_color.clone()
        } else if let (Some(r), Some(g), Some(b)) = (
            s[s.len() - 4].parse().ok(),
            s[s.len() - 3].parse().ok(),
            s[s.len() - 2].parse().ok(),
        ) {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
        };

        let end_time = if s.len() == 6 && has_end_color {
            start_time
        } else {
            match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Color Command token: {}", s[2]),
                    ))
                }
            }
        };

        Ok(ColorCommand {
            easing,
            start_time,
            end_time,
            start_color,
            end_color,
        })
    }
}
