#[derive(Debug, Clone)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Color { r, g, b }
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        let colors = s.splitn(3, ",").collect::<Vec<&str>>();
        Color {
            r: colors[0].parse().unwrap(),
            g: colors[1].parse().unwrap(),
            b: colors[2].parse().unwrap(),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{}", self.r, self.g, self.b)
    }
}
