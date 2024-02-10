mod command;
mod storyboard;

pub use command::Command;
use std::{ffi::OsString, path::PathBuf};
pub use storyboard::Storyboard;

use crate::Beatmap;

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

impl Beatmap {
    pub fn get_background_path(&self) -> String {
        OsString::from(PathBuf::from(self.get_directory()).join(self.events.get_background()))
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn get_video_path(&self) -> String {
        OsString::from(PathBuf::from(self.get_directory()).join(self.events.get_video()))
            .to_str()
            .unwrap()
            .to_string()
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
                    let file_name = OsString::from(&parts[2][1..parts[2].len() - 1]);

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
                let file_name = OsString::from(&parts[2][1..parts[2].len() - 1]);
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

    pub fn get_background(&self) -> OsString {
        if let Some(background) = &self.background {
            background.filename.clone()
        } else {
            OsString::new()
        }
    }

    pub fn get_video(&self) -> OsString {
        if let Some(video) = &self.video {
            video.filename.clone()
        } else {
            OsString::new()
        }
    }
}
