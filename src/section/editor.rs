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
        let mut display_string = String::from("[Editor]\n");
        if !self.bookmarks.is_empty() {
            display_string.push_str("Bookmarks: ");
            for (index, bookmark) in self.bookmarks.iter().enumerate() {
                if index == self.bookmarks.len() - 1 {
                    display_string.push_str(&format!("{}", bookmark));
                } else {
                    display_string.push_str(&format!("{},", bookmark));
                }
            }
            display_string.push_str("\n");
        }

        display_string.push_str(&format!("DistanceSpacing: {}\n", self.distance_spacing));
        display_string.push_str(&format!("BeatDivisor: {}\n", self.beat_divisor));
        display_string.push_str(&format!("GridSize: {}\n", self.grid_size));
        display_string.push_str(&format!("TimelineZoom: {}\n", self.timeline_zoom));

        write!(f, "{}", display_string)
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
