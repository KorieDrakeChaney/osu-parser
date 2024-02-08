use crate::token::EditorToken;
#[derive(Debug)]
pub struct Editor {
    bookmarks: Vec<i32>,
    distance_spacing: f32,
    beat_divisor: i32,
    grid_size: i32,
    timeline_zoom: f32,
}

impl Editor {
    pub fn parse(tokens: &Vec<EditorToken>) -> std::io::Result<Self> {
        let mut bookmarks = Vec::new();
        let mut distance_spacing = 0.0;
        let mut beat_divisor = 4;
        let mut grid_size = 4;
        let mut timeline_zoom = 1.0;

        for token in tokens {
            match token {
                EditorToken::Bookmarks(b) => {
                    for b in b.split(',') {
                        if let Ok(n) = b.parse() {
                            bookmarks.push(n);
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Invalid Editor token",
                            ));
                        }
                    }
                }
                EditorToken::DistanceSpacing(d) => distance_spacing = *d,
                EditorToken::BeatDivisor(b) => beat_divisor = *b,
                EditorToken::GridSize(g) => grid_size = *g,
                EditorToken::TimelineZoom(t) => timeline_zoom = *t,
            }
        }

        Ok(Editor {
            bookmarks,
            distance_spacing,
            beat_divisor,
            grid_size,
            timeline_zoom,
        })
    }
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
