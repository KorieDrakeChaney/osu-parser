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
        if s.len() < 9 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 9", s.len()),
            ));
        }

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

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[2]),
                ))
            }
        };

        let end_color = if let (Some(r), Some(g), Some(b)) =
            (s[8].parse().ok(), s[9].parse().ok(), s[10].parse().ok())
        {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
        };

        let start_color = if let (Some(r), Some(g), Some(b)) =
            (s[5].parse().ok(), s[6].parse().ok(), s[7].parse().ok())
        {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
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
