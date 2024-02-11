#[derive(Debug)]
pub struct Editor {
    bookmarks: Option<Vec<i32>>,
    distance_spacing: Option<f32>,
    beat_divisor: Option<i32>,
    grid_size: Option<i32>,
    timeline_zoom: Option<f32>,
}

impl std::fmt::Display for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "");
        }

        let mut display_string = String::from("[Editor]\n");

        if let Some(bookmarks) = &self.bookmarks {
            display_string.push_str("Bookmarks: ");
            for (index, bookmark) in bookmarks.iter().enumerate() {
                if index == bookmarks.len() - 1 {
                    display_string.push_str(&format!("{}", bookmark));
                } else {
                    display_string.push_str(&format!("{},", bookmark));
                }
            }
            display_string.push_str("\n");
        }

        if let Some(distance_spacing) = &self.distance_spacing {
            display_string.push_str(&format!("DistanceSpacing: {}\n", distance_spacing));
        }
        if let Some(beat_divisor) = &self.beat_divisor {
            display_string.push_str(&format!("BeatDivisor: {}\n", beat_divisor));
        }
        if let Some(grid_size) = &self.grid_size {
            display_string.push_str(&format!("GridSize: {}\n", grid_size));
        }
        if let Some(timeline_zoom) = &self.timeline_zoom {
            display_string.push_str(&format!("TimelineZoom: {}\n", timeline_zoom));
        }

        write!(f, "{}", display_string)
    }
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            bookmarks: None,
            distance_spacing: None,
            beat_divisor: None,
            grid_size: None,
            timeline_zoom: None,
        }
    }
}

impl Editor {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(':').map(|s| s.trim()).collect();
        let value = parts[1];

        match parts[0] {
            "Bookmarks" => {
                let mut bookmarks = Vec::new();
                for b in value.split(',') {
                    if let Ok(n) = b.parse() {
                        bookmarks.push(n);
                    }
                }
                self.bookmarks = Some(bookmarks);
            }
            "DistanceSpacing" => {
                if let Ok(n) = value.parse() {
                    self.distance_spacing = Some(n);
                }
            }
            "BeatDivisor" => {
                if let Ok(n) = value.parse() {
                    self.beat_divisor = Some(n);
                }
            }
            "GridSize" => {
                if let Ok(n) = value.parse() {
                    self.grid_size = Some(n);
                }
            }
            "TimelineZoom" => {
                if let Ok(n) = value.parse() {
                    self.timeline_zoom = Some(n);
                }
            }
            _ => {}
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bookmarks.is_none()
            && self.distance_spacing.is_none()
            && self.beat_divisor.is_none()
            && self.grid_size.is_none()
            && self.timeline_zoom.is_none()
    }
}
