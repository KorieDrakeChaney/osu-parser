use std::ffi::OsString;

use super::Command;

#[derive(Debug)]
pub enum StoryboardType {
    Sprite(SpriteType),
    Animation(AnimationType),
    Sample(SampleType),
}

impl From<&str> for StoryboardType {
    fn from(s: &str) -> Self {
        let parts = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        match parts[0] {
            "Sprite" => StoryboardType::Sprite(SpriteType::from(s)),
            "Animation" => StoryboardType::Animation(AnimationType::from(s)),
            "Sample" => StoryboardType::Sample(SampleType::from(s)),
            _ => todo!(),
        }
    }
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

impl From<&str> for SpriteType {
    fn from(s: &str) -> Self {
        let parts = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let layer = StoryboardLayer::from(parts[1]);
        let origin = Origin::from(parts[2]);
        let image_path = OsString::from(parts[3]);
        let offset_x = parts[4].parse().unwrap();
        let offset_y = parts[5].parse().unwrap();
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

impl From<&str> for AnimationType {
    fn from(s: &str) -> Self {
        let parts = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let layer = StoryboardLayer::from(parts[1]);
        let origin = Origin::from(parts[2]);
        let image_path = OsString::from(parts[3]);
        let offset_x = parts[4].parse().unwrap();
        let offset_y = parts[5].parse().unwrap();
        let frame_count = parts[6].parse().unwrap();
        let frame_delay = parts[7].parse().unwrap();
        let loop_type = LoopType::from(parts[8]);
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

impl From<&str> for SampleType {
    fn from(s: &str) -> Self {
        let parts = s.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
        let layer = StoryboardLayer::from(parts[1]);
        let image_path = OsString::from(parts[2]);
        let volume = parts[3].parse().unwrap();
        let time = parts[4].parse().unwrap();
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
            "{},{},{}",
            self.image_path.to_str().unwrap(),
            self.volume,
            self.time
        )
    }
}

#[derive(Debug)]
pub struct Storyboard {
    storyboard_type: StoryboardType,
    commands: Vec<Command>,
}

impl Storyboard {
    pub fn parse(value: &str, commands: Vec<Command>) -> std::io::Result<Self> {
        let parts = value.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

        let storyboard_type = StoryboardType::from(value);

        Ok(Storyboard {
            storyboard_type,
            commands,
        })
    }
}

impl std::fmt::Display for Storyboard {
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
