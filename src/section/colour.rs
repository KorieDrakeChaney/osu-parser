use crate::utils::Color;

#[derive(Debug)]
pub enum Colour {
    ComboColor(i32, Color),
    SliderTrackOverride(Color),
    SliderBorder(Color),
}

impl Colour {
    pub fn parse(s: &str) -> std::io::Result<Self> {
        let s: Vec<&str> = s.split(':').map(|s| s.trim()).collect();

        if s[0].contains("Combo") {
            Ok(Colour::ComboColor(
                s[0][5..].parse().unwrap(),
                Color::from(s[1]),
            ))
        } else if s[0] == "SliderTrackOverride" {
            Ok(Colour::SliderTrackOverride(Color::from(s[1])))
        } else if s[0] == "SliderBorder" {
            Ok(Colour::SliderBorder(Color::from(s[1])))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Colour token",
            ))
        }
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Colour::ComboColor(n, c) => write!(f, "Combo{}:{}\n", n, c),
            Colour::SliderTrackOverride(c) => write!(f, "SliderTrackOverride:{}\n", c),
            Colour::SliderBorder(c) => write!(f, "SliderBorder:{}\n", c),
        }
    }
}
