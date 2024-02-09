use std::ffi::OsString;

use super::colour::Color;

#[derive(Debug)]
pub struct OsuBackground {
    filename: OsString,
    x_offset: i32,
    y_offset: i32,
}

impl OsuBackground {
    pub fn new(filename: OsString, x_offset: i32, y_offset: i32) -> Self {
        OsuBackground {
            filename,
            x_offset,
            y_offset,
        }
    }
}

#[derive(Debug)]
pub struct OsuVideo {
    start_time: i32,
    filename: OsString,
    x_offset: i32,
    y_offset: i32,
}

impl OsuVideo {
    pub fn new(start_time: i32, filename: OsString, x_offset: i32, y_offset: i32) -> Self {
        OsuVideo {
            start_time,
            filename,
            x_offset,
            y_offset,
        }
    }
}

#[derive(Debug)]
pub struct OsuBreak {
    start_time: i32,
    end_time: i32,
}

impl OsuBreak {
    pub fn new(start_time: i32, end_time: i32) -> Self {
        OsuBreak {
            start_time,
            end_time,
        }
    }
}

#[derive(Debug)]
pub struct Events {
    background: Option<OsuBackground>,
    video: Option<OsuVideo>,
    breaks: Vec<OsuBreak>,
    storyboards: Vec<Storyboard>,
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Events]\n")?;
        if let Some(background) = &self.background {
            write!(
                f,
                "0,0,\"{}\",{},{}\n",
                background.filename.to_str().unwrap(),
                background.x_offset,
                background.y_offset
            )?;
        }
        if let Some(video) = &self.video {
            write!(
                f,
                "Video,{},\"{}\",{},{}\n",
                video.start_time,
                video.filename.to_str().unwrap(),
                video.x_offset,
                video.y_offset
            )?;
        }
        for osu_break in &self.breaks {
            write!(f, "2,{},{}\n", osu_break.start_time, osu_break.end_time)?;
        }
        for event in &self.storyboards {
            write!(f, "{}\n", event)?;
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum Effect {
    Additive,
    HorizontalFlip,
    VerticalFlip,
    OriginAtCenter,
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Effect::Additive => write!(f, "A"),
            Effect::HorizontalFlip => write!(f, "H"),
            Effect::VerticalFlip => write!(f, "V"),
            Effect::OriginAtCenter => write!(f, "R"),
        }
    }
}

impl From<&str> for Effect {
    fn from(s: &str) -> Self {
        match s {
            "A" => Effect::Additive,
            "H" => Effect::HorizontalFlip,
            "V" => Effect::VerticalFlip,
            "R" => Effect::OriginAtCenter,
            _ => Effect::Additive,
        }
    }
}
#[derive(Debug)]
pub enum Trigger {
    HitSoundClap,
    HitSoundFinish,
    HitSoundWhistle,
    Passing,
    Failing,
}

impl From<&str> for Trigger {
    fn from(s: &str) -> Self {
        match s {
            "HitSoundClap" => Trigger::HitSoundClap,
            "HitSoundFinish" => Trigger::HitSoundFinish,
            "HitSoundWhistle" => Trigger::HitSoundWhistle,
            "Passing" => Trigger::Passing,
            "Failing" => Trigger::Failing,
            _ => Trigger::HitSoundClap,
        }
    }
}

impl std::fmt::Display for Trigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Trigger::HitSoundClap => write!(f, "HitSoundClap"),
            Trigger::HitSoundFinish => write!(f, "HitSoundFinish"),
            Trigger::HitSoundWhistle => write!(f, "HitSoundWhistle"),
            Trigger::Passing => write!(f, "Passing"),
            Trigger::Failing => write!(f, "Failing"),
        }
    }
}

#[derive(Debug)]
pub struct FadeCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_opacity: f32,
    end_opacity: f32,
}

impl FadeCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_time = if s.len() == 4 {
            start_time
        } else {
            match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[2]),
                    ))
                }
            }
        };

        let end_opacity: f32 = if s.len() == 4 {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        } else {
            match s[4].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        let start_opacity: f32 = if s.len() == 4 {
            if s[2].is_empty() {
                end_opacity
            } else {
                match s[2].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Invalid Command token: {}", s[2]),
                        ));
                    }
                }
            }
        } else {
            if s[3].is_empty() {
                end_opacity
            } else {
                match s[3].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Invalid Command token: {}", s[3]),
                        ))
                    }
                }
            }
        };
        Ok(FadeCommand {
            easing,
            start_time,
            end_time,
            start_opacity,
            end_opacity,
        })
    }
}

impl std::fmt::Display for FadeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "F,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_opacity, self.end_opacity
        )
    }
}

#[derive(Debug)]
pub struct ScaleCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_scale: f32,
    end_scale: f32,
}

impl ScaleCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse::<i32>() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_time = if s.len() == 4 {
            start_time
        } else {
            match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[2]),
                    ))
                }
            }
        };

        let end_scale: f32 = if s.len() == 4 {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        } else {
            match s[4].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        let start_scale: f32 = if s.len() == 4 {
            if s[2].is_empty() {
                end_scale
            } else {
                match s[2].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Invalid Command token: {}", s[2]),
                        ));
                    }
                }
            }
        } else {
            if s[3].is_empty() {
                end_scale
            } else {
                match s[3].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Invalid Command token: {}", s[3]),
                        ))
                    }
                }
            }
        };

        Ok(ScaleCommand {
            easing,
            start_time,
            end_time,
            start_scale,
            end_scale,
        })
    }
}

impl std::fmt::Display for ScaleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "S,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_scale, self.end_scale
        )
    }
}

#[derive(Debug)]
pub struct VectorScaleCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_scale_x: i32,
    end_scale_x: i32,
    start_scale_y: i32,
    end_scale_y: i32,
}

impl std::fmt::Display for VectorScaleCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "V,{},{},{},{},{},{},{}\n",
            self.easing,
            self.start_time,
            self.end_time,
            self.start_scale_x,
            self.end_scale_x,
            self.start_scale_y,
            self.end_scale_y
        )
    }
}

impl VectorScaleCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 7", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[2]),
                ))
            }
        };

        let end_scale_y = match s[6].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[6]),
                ))
            }
        };

        let start_scale_y = if s[5].is_empty() {
            end_scale_y
        } else {
            match s[5].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[5]),
                    ))
                }
            }
        };

        let end_scale_x = match s[4].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[4]),
                ))
            }
        };

        let start_scale_x = if s[3].is_empty() {
            end_scale_x
        } else {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        Ok(VectorScaleCommand {
            easing,
            start_time,
            end_time,
            start_scale_x,
            end_scale_x,
            start_scale_y,
            end_scale_y,
        })
    }
}

#[derive(Debug)]
pub struct RotateCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_angle: i32,
    end_angle: i32,
}

impl std::fmt::Display for RotateCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "R,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_angle, self.end_angle
        )
    }
}

impl RotateCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 5", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[2]),
                ))
            }
        };

        let end_angle = match s[4].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[4]),
                ))
            }
        };

        let start_angle = if s[3].is_empty() {
            end_angle
        } else {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[3]),
                    ))
                }
            }
        };

        Ok(RotateCommand {
            easing,
            start_time,
            end_time,
            start_angle,
            end_angle,
        })
    }
}

#[derive(Debug)]
pub struct MoveCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
}

impl std::fmt::Display for MoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "M,{},{},{},{},{},{},{}\n",
            self.easing,
            self.start_time,
            self.end_time,
            self.start_x,
            self.start_y,
            self.end_x,
            self.end_y
        )
    }
}

impl MoveCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let mut start_x: f32 = 0.0;
        let mut start_y: f32 = 0.0;
        let mut end_time = 0;

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[1]),
                ))
            }
        };

        let end_index = s.len() - 1;

        let end_x = match s[end_index - 1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[end_index - 2]),
                ))
            }
        };

        let end_y = match s[end_index].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Command token: {}", s[end_index]),
                ))
            }
        };

        if s[end_index - 2].is_empty() && end_index - 3 == 1 {
            start_x = end_x;
            start_y = end_y;
            end_time = start_time;
        } else if s[end_index - 2].is_empty() {
            start_y = end_y;
            end_time = start_time;
            start_x = match s[end_index - 3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 4]),
                    ))
                }
            }
        } else {
            end_time = match s[2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 1]),
                    ))
                }
            };
            start_y = match s[end_index - 2].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 2]),
                    ))
                }
            };
            start_x = match s[end_index - 3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid Command token: {}", s[end_index - 3]),
                    ))
                }
            };
        }

        Ok(MoveCommand {
            easing,
            start_time,
            end_time,
            start_x,
            start_y,
            end_x,
            end_y,
        })
    }
}

#[derive(Debug)]
pub struct MoveXCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_x: i32,
    end_x: i32,
}

impl std::fmt::Display for MoveXCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MX,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_x, self.end_x
        )
    }
}

impl MoveXCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let mut parts = s.iter().collect::<Vec<&&str>>();

        let mut index = 0;

        let easing = match parts.get(index) {
            Some(x) => {
                index += 1;
                x.parse().unwrap()
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid MoveX Command token: {}", parts.get(index).unwrap()),
            ))?,
        };

        let start_time = match parts.get(index) {
            Some(x) => {
                index += 1;
                x.parse().unwrap()
            }
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid MoveX Command token: {}", parts.get(index).unwrap()),
            ))?,
        };

        let end_time = match parts.get(index) {
            Some(x) => x.parse().unwrap(),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid MoveX Command token: {}", parts.get(index).unwrap()),
            ))?,
        };

        let end_x = if parts.len() > 3 {
            parts.pop().unwrap().parse().unwrap()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid MoveX Data"),
            ))?
        };

        let start_x = if parts.len() > 3 {
            parts.pop().unwrap().parse().unwrap()
        } else {
            end_x
        };

        Ok(MoveXCommand {
            easing,
            start_time,
            end_time,
            start_x,
            end_x,
        })
    }
}

#[derive(Debug)]
pub struct MoveYCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_y: i32,
    end_y: i32,
}

impl std::fmt::Display for MoveYCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MY,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_y, self.end_y
        )
    }
}

impl MoveYCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 5 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 5", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveY Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveY Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveY Command token: {}", s[2]),
                ))
            }
        };

        let end_y = match s[4].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid MoveY Command token: {}", s[4]),
                ))
            }
        };

        let start_y = if s[3].is_empty() {
            end_y
        } else {
            match s[3].parse() {
                Ok(x) => x,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid MoveY Command token: {}", s[3]),
                    ))
                }
            }
        };

        Ok(MoveYCommand {
            easing,
            start_time,
            end_time,
            start_y,
            end_y,
        })
    }
}

#[derive(Debug)]
pub struct ColorCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    start_color: Color,
    end_color: Color,
}

impl std::fmt::Display for ColorCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "C,{},{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.start_color, self.end_color
        )
    }
}

impl ColorCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 9 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 9", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Color Command token: {}", s[2]),
                ))
            }
        };

        let end_color = if let (Some(r), Some(g), Some(b)) =
            (s[8].parse().ok(), s[9].parse().ok(), s[10].parse().ok())
        {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
        };

        let start_color = if let (Some(r), Some(g), Some(b)) =
            (s[5].parse().ok(), s[6].parse().ok(), s[7].parse().ok())
        {
            Color::new(r, g, b)
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid Color Command Data",
            ));
        };

        Ok(ColorCommand {
            easing,
            start_time,
            end_time,
            start_color,
            end_color,
        })
    }
}

#[derive(Debug)]
pub struct ParameterCommand {
    easing: i32,
    start_time: i32,
    end_time: i32,
    parameter_type: Effect,
}

impl std::fmt::Display for ParameterCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "P,{},{},{},{}\n",
            self.easing, self.start_time, self.end_time, self.parameter_type
        )
    }
}

impl ParameterCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 4", s.len()),
            ));
        }

        let easing = match s[0].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[0]),
                ))
            }
        };

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Parameter Command token: {}", s[2]),
                ))
            }
        };

        let parameter_type = Effect::from(s[3]);

        Ok(ParameterCommand {
            easing,
            start_time,
            end_time,
            parameter_type,
        })
    }
}

#[derive(Debug)]
pub struct TriggerCommand {
    trigger_type: Trigger,
    start_time: i32,
    end_time: i32,
    group: Option<String>,
}

impl std::fmt::Display for TriggerCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut display_string = format!(
            "T,{},{},{}",
            self.trigger_type, self.start_time, self.end_time
        );

        if let Some(group) = &self.group {
            display_string.push_str(&format!(",{}", group));
        }

        write!(f, "{}\n", display_string)
    }
}

impl TriggerCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        if s.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid length of {}, needs 4", s.len()),
            ));
        }

        let trigger_type = Trigger::from(s[0]);

        let start_time = match s[1].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Trigger Command token: {}", s[1]),
                ))
            }
        };

        let end_time = match s[2].parse() {
            Ok(x) => x,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid Trigger Command token: {}", s[2]),
                ))
            }
        };

        let group = if s.len() > 3 {
            Some(s[3].to_string())
        } else {
            None
        };

        Ok(TriggerCommand {
            trigger_type,
            start_time,
            end_time,
            group,
        })
    }
}

#[derive(Debug)]
pub struct LoopCommand {
    start_time: i32,
    loop_count: i32,
}

impl std::fmt::Display for LoopCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "L,{},{}\n", self.start_time, self.loop_count)
    }
}

impl LoopCommand {
    pub fn parse(s: &[&str]) -> std::io::Result<Self> {
        let start_time = s[0].parse().unwrap();
        let loop_count = s[1].parse().unwrap();
        Ok(LoopCommand {
            start_time,
            loop_count,
        })
    }
}

#[derive(Debug)]
pub enum Command {
    Fade(FadeCommand),
    Scale(ScaleCommand),
    VectorScale(VectorScaleCommand),
    Rotate(RotateCommand),
    Move(MoveCommand),
    MoveX(MoveXCommand),
    MoveY(MoveYCommand),
    Color(ColorCommand),
    Parameter(ParameterCommand),
    Loop(LoopCommand),
    Trigger(TriggerCommand),
}

impl Command {
    pub fn parse(s: &str) -> std::io::Result<Self> {
        let parts = s.split(",").collect::<Vec<&str>>();
        match parts[0] {
            "F" => Ok(Command::Fade(FadeCommand::parse(&parts[1..])?)),
            "S" => Ok(Command::Scale(ScaleCommand::parse(&parts[1..])?)),
            "V" => Ok(Command::VectorScale(VectorScaleCommand::parse(
                &parts[1..],
            )?)),
            "R" => Ok(Command::Rotate(RotateCommand::parse(&parts[1..])?)),
            "M" => Ok(Command::Move(MoveCommand::parse(&parts[1..])?)),
            "MX" => Ok(Command::MoveX(MoveXCommand::parse(&parts[1..])?)),
            "MY" => Ok(Command::MoveY(MoveYCommand::parse(&parts[1..])?)),
            "C" => Ok(Command::Color(ColorCommand::parse(&parts[1..])?)),
            "P" => Ok(Command::Parameter(ParameterCommand::parse(&parts[1..])?)),
            "L" => Ok(Command::Loop(LoopCommand::parse(&parts[1..])?)),
            "T" => Ok(Command::Trigger(TriggerCommand::parse(&parts[1..])?)),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid Command token: {}", parts[0]),
            )),
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Command::Fade(fade_command) => write!(f, "{}", fade_command),
            Command::Scale(scale_command) => write!(f, "{}", scale_command),
            Command::VectorScale(vector_scale_command) => write!(f, "{}", vector_scale_command),
            Command::Rotate(rotate_command) => write!(f, "{}", rotate_command),
            Command::Move(move_command) => write!(f, "{}", move_command),
            Command::MoveX(move_x_command) => write!(f, "{}", move_x_command),
            Command::MoveY(move_y_command) => write!(f, "{}", move_y_command),
            Command::Color(color_command) => write!(f, "{}", color_command),
            Command::Parameter(parameter_command) => write!(f, "{}", parameter_command),
            Command::Loop(loop_command) => write!(f, "{}", loop_command),
            Command::Trigger(trigger_command) => write!(f, "{}", trigger_command),
        }
    }
}
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
            "Storyboard {},{},{},{},{},{}\n",
            self.storyboard_type,
            self.layer,
            self.origin,
            self.image_path.to_str().unwrap(),
            self.offset_x,
            self.offset_y
        );
        for command in &self.commands {
            display_string += &format!("{}", command);
        }
        write!(f, "{}", display_string)
    }
}

impl Default for Events {
    fn default() -> Self {
        Events {
            background: None,
            video: None,
            breaks: Vec::new(),
            storyboards: Vec::new(),
        }
    }
}

impl Events {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
        match parts[0] {
            "0" => {
                if parts[1] == "0" {
                    let file_name = OsString::from(parts[2]);

                    let x_offset = if parts.len() > 3 {
                        parts[3].parse().unwrap()
                    } else {
                        0_i32
                    };
                    let y_offset = if parts.len() > 4 {
                        parts[4].parse().unwrap()
                    } else {
                        0_i32
                    };
                    self.background = Some(OsuBackground::new(file_name, x_offset, y_offset));
                }
            }
            "1" | "Video" => {
                let start_time = parts[1].parse().unwrap();
                let file_name = OsString::from(parts[2]);
                let x_offset = if parts.len() > 3 {
                    parts[3].parse().unwrap()
                } else {
                    0_i32
                };
                let y_offset = if parts.len() > 4 {
                    parts[4].parse().unwrap()
                } else {
                    0_i32
                };

                self.video = Some(OsuVideo::new(start_time, file_name, x_offset, y_offset));
            }
            "2" | "Break" => {
                let start_time = parts[1].parse().unwrap();
                let end_time = parts[2].parse().unwrap();

                self.breaks.push(OsuBreak::new(start_time, end_time));
            }
            _ => {}
        }
    }

    pub fn push_storyboard(&mut self, storyboard: Storyboard) {
        self.storyboards.push(storyboard);
    }
}
