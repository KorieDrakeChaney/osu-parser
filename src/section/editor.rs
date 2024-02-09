#[derive(Debug)]
pub struct Editor {
    bookmarks: Vec<i32>,
    distance_spacing: f32,
    beat_divisor: i32,
    grid_size: i32,
    timeline_zoom: f32,
}

impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Editor]\n")?;
        write!(f, "Bookmarks: ")?;
        for b in &self.bookmarks {
            write!(f, "{},", b)?;
        }
        write!(f, "\n")?;
        write!(f, "DistanceSpacing: {}\n", self.distance_spacing)?;
        write!(f, "BeatDivisor: {}\n", self.beat_divisor)?;
        write!(f, "GridSize: {}\n", self.grid_size)?;
        write!(f, "TimelineZoom: {}\n", self.timeline_zoom)
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            bookmarks: Vec::new(),
            distance_spacing: 0.0,
            beat_divisor: 4,
            grid_size: 4,
            timeline_zoom: 1.0,
        }
    }
}

impl Editor {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];

        match parts[0] {
            "Bookmarks" => {
                for b in value.split(',') {
                    if let Ok(n) = b.parse() {
                        self.bookmarks.push(n);
                    }
                }
            }
            "DistanceSpacing" => {
                if let Ok(n) = value.parse() {
                    self.distance_spacing = n;
                }
            }
            "BeatDivisor" => {
                if let Ok(n) = value.parse() {
                    self.beat_divisor = n;
                }
            }
            "GridSize" => {
                if let Ok(n) = value.parse() {
                    self.grid_size = n;
                }
            }
            "TimelineZoom" => {
                if let Ok(n) = value.parse() {
                    self.timeline_zoom = n;
                }
            }
            _ => {}
        }
    }
}
