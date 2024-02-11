use std::ffi::OsString;

use super::Command;

#[derive(Debug)]
pub enum StoryboardType {
    Sprite,
    Animation,
}

impl From<&str> for StoryboardType {
    fn from(s: &str) -> Self {
        match s {
            "Sprite" => StoryboardType::Sprite,
            "Animation" => StoryboardType::Animation,
            _ => StoryboardType::Sprite,
        }
    }
}

impl std::fmt::Display for StoryboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StoryboardType::Sprite => write!(f, "Sprite"),
            StoryboardType::Animation => write!(f, "Animation"),
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
pub struct Storyboard {
    storyboard_type: StoryboardType,
    layer: StoryboardLayer,
    origin: Origin,
    image_path: OsString,
    offset_x: f32,
    offset_y: f32,
    commands: Vec<Command>,
}

impl Storyboard {
    pub fn parse(value: &str, commands: Vec<Command>) -> std::io::Result<Self> {
        let parts = value.split(',').map(|s| s.trim()).collect::<Vec<&str>>();

        let mut image_path = OsString::new();
        let mut offset_x: f32 = 0.0;
        let mut offset_y: f32 = 0.0;

        let storyboard_type = StoryboardType::from(parts[0]);
        let layer = StoryboardLayer::from(parts[1]);
        let origin = Origin::from(parts[2]);
        image_path.push(parts[3]);

        if parts.len() > 4 {
            offset_x = parts[4].parse().unwrap();
            offset_y = parts[5].parse().unwrap();
        }

        Ok(Storyboard {
            storyboard_type,
            layer,
            origin,
            image_path,
            offset_x,
            offset_y,
            commands,
        })
    }
}

impl std::fmt::Display for Storyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "{},{},{},{},{},{}\n",
            self.storyboard_type,
            self.layer,
            self.origin,
            self.image_path.to_str().unwrap(),
            self.offset_x,
            self.offset_y
        );

        let scope = 0; // TODO: Implement scope for nested commands
        for command in &self.commands {
            display_string += &" ".repeat(scope + 1);
            display_string += &format!("{}", command);
        }
        write!(f, "{}", display_string)
    }
}
