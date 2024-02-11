use std::ffi::OsString;

use super::Command;

#[derive(Debug)]
pub enum StoryboardType {
    Sprite(SpriteType),
    Animation(AnimationType),
    Sample(SampleType),
}

impl std::fmt::Display for StoryboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StoryboardType::Sprite(sprite) => write!(f, "Sprite,{}", sprite),
            StoryboardType::Animation(animation) => write!(f, "Animation,{}", animation),
            StoryboardType::Sample(sample) => write!(f, "Sample,{}", sample),
        }
    }
}

#[derive(Debug)]
pub enum StoryboardLayer {
    Background,
    Fail,
    Pass,
    Foreground,
    Overlay,
}

impl From<&str> for StoryboardLayer {
    fn from(s: &str) -> Self {
        match s {
            "Background" => StoryboardLayer::Background,
            "Fail" => StoryboardLayer::Fail,
            "Pass" => StoryboardLayer::Pass,
            "Foreground" => StoryboardLayer::Foreground,
            "Overlay" => StoryboardLayer::Overlay,
            _ => StoryboardLayer::Background,
        }
    }
}

impl std::fmt::Display for StoryboardLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StoryboardLayer::Background => write!(f, "Background"),
            StoryboardLayer::Fail => write!(f, "Fail"),
            StoryboardLayer::Pass => write!(f, "Pass"),
            StoryboardLayer::Foreground => write!(f, "Foreground"),
            StoryboardLayer::Overlay => write!(f, "Overlay"),
        }
    }
}

#[derive(Debug)]
pub enum LoopType {
    LoopForever,
    LoopOnce,
}

impl From<&str> for LoopType {
    fn from(s: &str) -> Self {
        match s {
            "LoopForever" => LoopType::LoopForever,
            "LoopOnce" => LoopType::LoopOnce,
            _ => LoopType::LoopForever,
        }
    }
}

impl std::fmt::Display for LoopType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LoopType::LoopForever => write!(f, "LoopForever"),
            LoopType::LoopOnce => write!(f, "LoopOnce"),
        }
    }
}

#[derive(Debug)]
pub enum Origin {
    TopLeft,
    TopCentre,
    TopRight,
    CentreLeft,
    Centre,
    CentreRight,
    BottomLeft,
    BottomCentre,
    BottomRight,
}

impl From<&str> for Origin {
    fn from(s: &str) -> Self {
        match s {
            "TopLeft" => Origin::TopLeft,
            "TopCentre" => Origin::TopCentre,
            "TopRight" => Origin::TopRight,
            "CentreLeft" => Origin::CentreLeft,
            "Centre" => Origin::Centre,
            "CentreRight" => Origin::CentreRight,
            "BottomLeft" => Origin::BottomLeft,
            "BottomCentre" => Origin::BottomCentre,
            "BottomRight" => Origin::BottomRight,
            _ => Origin::TopLeft,
        }
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Origin::TopLeft => write!(f, "TopLeft"),
            Origin::TopCentre => write!(f, "TopCentre"),
            Origin::TopRight => write!(f, "TopRight"),
            Origin::CentreLeft => write!(f, "CentreLeft"),
            Origin::Centre => write!(f, "Centre"),
            Origin::CentreRight => write!(f, "CentreRight"),
            Origin::BottomLeft => write!(f, "BottomLeft"),
            Origin::BottomCentre => write!(f, "BottomCentre"),
            Origin::BottomRight => write!(f, "BottomRight"),
        }
    }
}

#[derive(Debug)]
pub struct SpriteType {
    layer: StoryboardLayer,
    origin: Origin,
    image_path: OsString,
    offset_x: f32,
    offset_y: f32,
}

impl SpriteType {
    pub fn new(
        layer: StoryboardLayer,
        origin: Origin,
        image_path: OsString,
        offset_x: f32,
        offset_y: f32,
    ) -> Self {
        SpriteType {
            layer,
            origin,
            image_path,
            offset_x,
            offset_y,
        }
    }
}

impl std::fmt::Display for SpriteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sprite,{},{},{},{},{}",
            self.layer,
            self.origin,
            self.image_path.to_str().unwrap(),
            self.offset_x,
            self.offset_y
        )
    }
}

#[derive(Debug)]
pub struct AnimationType {
    layer: StoryboardLayer,
    origin: Origin,
    image_path: OsString,
    offset_x: f32,
    offset_y: f32,
    frame_count: i32,
    frame_delay: i32,
    loop_type: LoopType,
}

impl AnimationType {
    pub fn new(
        layer: StoryboardLayer,
        origin: Origin,
        image_path: OsString,
        offset_x: f32,
        offset_y: f32,
        frame_count: i32,
        frame_delay: i32,
        loop_type: LoopType,
    ) -> Self {
        AnimationType {
            layer,
            origin,
            image_path,
            offset_x,
            offset_y,
            frame_count,
            frame_delay,
            loop_type,
        }
    }
}

impl std::fmt::Display for AnimationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Animation,{},{},{},{},{},{},{},{}",
            self.layer,
            self.origin,
            self.image_path.to_str().unwrap(),
            self.offset_x,
            self.offset_y,
            self.frame_count,
            self.frame_delay,
            self.loop_type
        )
    }
}

#[derive(Debug)]
pub struct SampleType {
    layer: StoryboardLayer,
    image_path: OsString,
    volume: i32,
    time: i32,
}

impl SampleType {
    pub fn new(layer: StoryboardLayer, image_path: OsString, volume: i32, time: i32) -> Self {
        SampleType {
            layer,
            image_path,
            volume,
            time,
        }
    }
}

impl std::fmt::Display for SampleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Sample,{},{},{},{}",
            self.layer,
            self.image_path.to_str().unwrap(),
            self.volume,
            self.time
        )
    }
}

#[derive(Debug)]
pub struct OsuStoryboard {
    storyboard_type: StoryboardType,
    commands: Vec<Command>,
}

impl OsuStoryboard {
    pub fn parse(value: &str) -> std::io::Result<Self> {
        let parts = value.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

        let storyboard_type = match parts[0] {
            "Sprite" => {
                let layer = if parts.len() > 1 {
                    StoryboardLayer::from(parts[1])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let origin = if parts.len() > 2 {
                    Origin::from(parts[2])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let image_path = if parts.len() > 3 {
                    OsString::from(parts[3])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let offset_x = if parts.len() > 4 {
                    parts[4].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let offset_y = if parts.len() > 5 {
                    parts[5].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                StoryboardType::Sprite(SpriteType::new(
                    layer, origin, image_path, offset_x, offset_y,
                ))
            }
            "Animation" => {
                let layer = if parts.len() > 1 {
                    StoryboardLayer::from(parts[1])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let origin = if parts.len() > 2 {
                    Origin::from(parts[2])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let image_path = if parts.len() > 3 {
                    OsString::from(parts[3])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let offset_x = if parts.len() > 4 {
                    parts[4].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let offset_y = if parts.len() > 5 {
                    parts[5].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let frame_count = if parts.len() > 6 {
                    parts[6].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let frame_delay = if parts.len() > 7 {
                    parts[7].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let loop_type = if parts.len() > 8 {
                    LoopType::from(parts[8])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                StoryboardType::Animation(AnimationType::new(
                    layer,
                    origin,
                    image_path,
                    offset_x,
                    offset_y,
                    frame_count,
                    frame_delay,
                    loop_type,
                ))
            }
            "Sample" => {
                let layer = if parts.len() > 1 {
                    StoryboardLayer::from(parts[1])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let image_path = if parts.len() > 2 {
                    OsString::from(parts[2])
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let volume = if parts.len() > 3 {
                    parts[3].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                let time = if parts.len() > 4 {
                    parts[4].parse().unwrap()
                } else {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid data from {}", value),
                    ));
                };

                StoryboardType::Sample(SampleType::new(layer, image_path, volume, time))
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid Storyboard token",
                ))
            }
        };

        Ok(OsuStoryboard {
            storyboard_type,
            commands: Vec::new(),
        })
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}

impl std::fmt::Display for OsuStoryboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = match &self.storyboard_type {
            StoryboardType::Sprite(sprite) => format!("{}\n", sprite),
            StoryboardType::Animation(animation) => format!("{}\n", animation),
            StoryboardType::Sample(sample) => format!("{}\n", sample),
        };

        let scope = 0; // TODO: Implement scope for nested commands
        for command in &self.commands {
            display_string += &" ".repeat(scope + 1);
            display_string += &format!("{}", command);
        }
        write!(f, "{}", display_string)
    }
}
