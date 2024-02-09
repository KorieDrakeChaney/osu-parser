use std::ffi::OsString;

#[derive(Debug)]
pub struct HitSample {
    normal_set: i32,
    addition_set: i32,
    index: i32,
    volume: i32,
    filename: Option<OsString>,
}

impl HitSample {
    pub fn parse(s: &str) -> std::io::Result<Self> {
        let samples = s.split(":").collect::<Vec<&str>>();
        Ok(HitSample {
            normal_set: samples[0].parse().unwrap(),
            addition_set: if samples.len() > 1 {
                samples[1].parse().unwrap()
            } else {
                0
            },
            index: if samples.len() > 2 {
                samples[2].parse().unwrap()
            } else {
                0
            },
            volume: if samples.len() > 3 {
                samples[3].parse().unwrap()
            } else {
                0
            },
            filename: if samples.len() > 4 {
                Some(OsString::from(samples[4]))
            } else {
                None
            },
        })
    }
}

impl Default for HitSample {
    fn default() -> Self {
        HitSample {
            normal_set: 0,
            addition_set: 0,
            index: 0,
            volume: 0,
            filename: None,
        }
    }
}

impl std::fmt::Display for HitSample {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(filename) = &self.filename {
            write!(
                f,
                "{}:{}:{}:{}:{}",
                self.normal_set,
                self.addition_set,
                self.index,
                self.volume,
                filename.to_str().unwrap()
            )
        } else {
            write!(
                f,
                "{}:{}:{}:{}:",
                self.normal_set, self.addition_set, self.index, self.volume
            )
        }
    }
}
#[derive(Debug)]
pub struct EdgeSet {
    normal_set: i32,
    addition_set: i32,
}

impl EdgeSet {
    pub fn parse(s: &str) -> std::io::Result<Self> {
        let mut iter = s.split(":");
        Ok(EdgeSet {
            normal_set: iter.next().unwrap().parse().unwrap(),
            addition_set: iter.next().unwrap().parse().unwrap(),
        })
    }
}

impl std::fmt::Display for EdgeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.normal_set, self.addition_set)
    }
}
#[derive(Debug)]
pub struct CircleHitObject {
    x: i32,
    y: i32,
    time: i32,
    object_type: i32,
    hit_sound: i32,
    params: Vec<String>,
    hit_sample: Option<HitSample>,
}

impl CircleHitObject {
    pub fn parse(s: &[&str], x: i32, y: i32, time: i32, object_type: i32) -> std::io::Result<Self> {
        let mut params: Vec<String> = Vec::new();
        let mut hit_sample = None;

        let hit_sound = if let Ok(n) = s[4].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid CircleHitObject token",
            ));
        };

        if s.len() > 6 {
            for p in &s[5..s.len() - 1] {
                params.push(p.to_string());
            }
        }

        if s[s.len() - 1].contains(":") {
            hit_sample = Some(HitSample::parse(s[s.len() - 1])?);
        } else if s.len() > 6 {
            params.push(s[s.len() - 1].to_string());
        }

        Ok(CircleHitObject {
            x,
            y,
            time,
            object_type,
            hit_sound,
            params,
            hit_sample,
        })
    }
}

impl std::fmt::Display for CircleHitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "{},{},{},{},{}",
            self.x, self.y, self.time, self.object_type, self.hit_sound
        );

        for p in &self.params {
            display_string += &format!("{},", p);
        }

        match &self.hit_sample {
            Some(h) => display_string += &format!(",{}", h.to_string()),
            None => {}
        }

        write!(f, "{}", display_string)
    }
}
#[derive(Debug)]
pub enum CurveType {
    Bezier,
    CentripetalCatmullRom,
    Linear,
    Perfect,
}

impl From<&str> for CurveType {
    fn from(s: &str) -> Self {
        match s {
            "B" => CurveType::Bezier,
            "C" => CurveType::CentripetalCatmullRom,
            "L" => CurveType::Linear,
            "P" => CurveType::Perfect,
            _ => CurveType::Linear,
        }
    }
}

impl std::fmt::Display for CurveType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CurveType::Bezier => write!(f, "B"),
            CurveType::CentripetalCatmullRom => write!(f, "C"),
            CurveType::Linear => write!(f, "L"),
            CurveType::Perfect => write!(f, "P"),
        }
    }
}
#[derive(Debug)]
pub struct CurvePoint {
    x: i32,
    y: i32,
}

impl From<&str> for CurvePoint {
    fn from(s: &str) -> Self {
        let mut iter = s.split(":");
        CurvePoint {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }
}

impl std::fmt::Display for CurvePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.x, self.y)
    }
}
#[derive(Debug)]
pub struct SliderHitObject {
    x: i32,
    y: i32,
    time: i32,
    object_type: i32,
    hit_sound: i32,
    curve_type: CurveType,
    curve_points: Vec<CurvePoint>,
    slides: i32,
    length: f32,
    edge_sounds: Vec<i32>,
    edge_sets: Vec<EdgeSet>,
    hit_sample: Option<HitSample>,
}

impl SliderHitObject {
    pub fn parse(s: &[&str], x: i32, y: i32, time: i32, object_type: i32) -> std::io::Result<Self> {
        let mut curve_points = Vec::new();
        let mut edge_sounds = Vec::new();
        let mut edge_sets = Vec::new();
        let mut hit_sample = None;

        let hit_sound = if let Ok(n) = s[4].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid SliderHitObject token",
            ));
        };

        let mut iter = s[5].split("|");
        let curve_type = CurveType::from(iter.next().unwrap());
        while let Some(point) = iter.next() {
            curve_points.push(CurvePoint::from(point));
        }

        let slides = if let Ok(n) = s[6].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid SliderHitObject token",
            ));
        };

        let length = if let Ok(n) = s[7].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid SliderHitObject token",
            ));
        };

        if s.len() > 8 {
            let sounds = s[8].split("|");
            for s in sounds {
                edge_sounds.push(s.parse().unwrap());
            }
        }

        if s.len() > 9 {
            let sets = s[9].split("|");
            for s in sets {
                edge_sets.push(EdgeSet::parse(s)?);
            }
        }

        if s.len() > 10 {
            if s[10].contains(":") {
                hit_sample = Some(HitSample::parse(s[10])?);
            }
        }

        Ok(SliderHitObject {
            x,
            y,
            time,
            object_type,
            hit_sound,
            curve_type,
            curve_points,
            slides,
            length,
            edge_sounds,
            edge_sets,
            hit_sample,
        })
    }
}

impl std::fmt::Display for SliderHitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "{},{},{},{},{},{}",
            self.x, self.y, self.time, self.object_type, self.hit_sound, self.curve_type
        );
        for point in &self.curve_points {
            display_string += &format!("|{}", point);
        }

        display_string += &format!(",{},{}", self.slides, self.length);

        for (index, sound) in self.edge_sounds.iter().enumerate() {
            if index == 0 {
                display_string.push(',');
            }
            display_string += &sound.to_string();
            if index != self.edge_sounds.len() - 1 {
                display_string.push('|');
            }
        }

        for (index, set) in self.edge_sets.iter().enumerate() {
            if index == 0 {
                display_string.push(',');
            }
            display_string += &set.to_string();
            if index != self.edge_sets.len() - 1 {
                display_string.push('|');
            }
        }

        match &self.hit_sample {
            Some(h) => display_string += &format!(",{}", h.to_string()),
            None => {}
        }

        write!(f, "{}", display_string)
    }
}
#[derive(Debug)]
pub struct SpinnerHitObject {
    x: i32,
    y: i32,
    time: i32,
    object_type: i32,
    hit_sound: i32,
    end_time: i32,
    hit_sample: Option<HitSample>,
}

impl SpinnerHitObject {
    pub fn parse(s: &[&str], x: i32, y: i32, time: i32, object_type: i32) -> std::io::Result<Self> {
        let mut hit_sample = None;

        let hit_sound = if let Ok(n) = s[4].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid SpinnerHitObject token",
            ));
        };

        let end_time = if let Ok(n) = s[5].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid SpinnerHitObject token",
            ));
        };

        if s.len() > 6 {
            if s[6].contains(":") {
                hit_sample = Some(HitSample::parse(s[6])?);
            }
        }

        Ok(SpinnerHitObject {
            x,
            y,
            time,
            object_type,
            hit_sound,
            end_time,
            hit_sample,
        })
    }
}

impl std::fmt::Display for SpinnerHitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "{},{},{},{},{},{}",
            self.x, self.y, self.time, self.object_type, self.hit_sound, self.end_time,
        );

        match &self.hit_sample {
            Some(h) => display_string += &format!(",{}", h.to_string()),
            None => {}
        }

        write!(f, "{}", display_string)
    }
}
#[derive(Debug)]
pub struct HoldHitObject {
    x: i32,
    y: i32,
    time: i32,
    object_type: i32,
    hit_sound: i32,
    end_time: i32,
    hit_sample: Option<HitSample>,
}

impl HoldHitObject {
    pub fn parse(s: &[&str], x: i32, y: i32, time: i32, object_type: i32) -> std::io::Result<Self> {
        let mut hit_sample = None;
        let mut end_time = 0;

        let hit_sound = if let Ok(n) = s[4].parse() {
            n
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid HoldHitObject token",
            ));
        };

        if s.len() > 5 {
            if s[5].contains(":") {
                let mut split = s[5].splitn(2, ':');
                end_time = split.next().unwrap().parse().unwrap();
                hit_sample = Some(HitSample::parse(split.next().unwrap())?);
            }
        }

        Ok(HoldHitObject {
            x,
            y,
            time,
            object_type,
            hit_sound,
            end_time,
            hit_sample,
        })
    }
}

impl std::fmt::Display for HoldHitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "{},{},{},{},{},{}",
            self.x, self.y, self.time, self.object_type, self.hit_sound, self.end_time,
        );

        match &self.hit_sample {
            Some(h) => display_string += &format!(":{}", h.to_string()),
            None => {}
        }

        write!(f, "{}", display_string)
    }
}
#[derive(Debug)]
pub enum HitObject {
    Circle(CircleHitObject),
    Slider(SliderHitObject),
    Spinner(SpinnerHitObject),
    Hold(HoldHitObject),
}

impl HitObject {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let x = s[0].parse().unwrap();
        let y = s[1].parse().unwrap();
        let time = s[2].parse().unwrap();
        let object_type = s[3].parse().unwrap();

        if object_type & 1 != 0 {
            Ok(HitObject::Circle(CircleHitObject::parse(
                s,
                x,
                y,
                time,
                object_type,
            )?))
        } else if object_type & 2 != 0 {
            Ok(HitObject::Slider(SliderHitObject::parse(
                s,
                x,
                y,
                time,
                object_type,
            )?))
        } else if object_type & 8 != 0 {
            Ok(HitObject::Spinner(SpinnerHitObject::parse(
                s,
                x,
                y,
                time,
                object_type,
            )?))
        } else if object_type & 128 != 0 {
            Ok(HitObject::Hold(HoldHitObject::parse(
                s,
                x,
                y,
                time,
                object_type,
            )?))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid HitObject token",
            ))
        }
    }
}

impl std::fmt::Display for HitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HitObject::Circle(c) => write!(f, "{}\n", c),
            HitObject::Slider(s) => write!(f, "{}\n", s),
            HitObject::Spinner(s) => write!(f, "{}\n", s),
            HitObject::Hold(h) => write!(f, "{}\n", h),
        }
    }
}
