mod command;
mod storyboard;

use std::{ffi::OsString, path::PathBuf};

pub use command::Command;
pub use storyboard::OsuStoryboard;

use crate::Beatmap;

#[derive(Debug)]
pub struct OsuBackground {
    start_time: i32,
    filename: String,
    x_offset: Option<f32>,
    y_offset: Option<f32>,
}

impl OsuBackground {
    pub fn new(
        start_time: i32,
        filename: String,
        x_offset: Option<f32>,
        y_offset: Option<f32>,
    ) -> Self {
        OsuBackground {
            start_time,
            filename,
            x_offset,
            y_offset,
        }
    }
}

#[derive(Debug)]
pub struct OsuVideo {
    start_time: i32,
    filename: String,
    x_offset: Option<f32>,
    y_offset: Option<f32>,
}

impl OsuVideo {
    pub fn new(
        start_time: i32,
        filename: String,
        x_offset: Option<f32>,
        y_offset: Option<f32>,
    ) -> Self {
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
pub struct OsuEvent {
    event_type: String,
    start_time: i32,
    params: Option<Vec<String>>,
}

impl OsuEvent {
    pub fn new(event_type: String, start_time: i32, params: Option<Vec<String>>) -> Self {
        OsuEvent {
            event_type,
            start_time,
            params,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Background(OsuBackground),
    Video(OsuVideo),
    Break(OsuBreak),
    Storyboard(OsuStoryboard),
    Basic(OsuEvent),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Event::Background(background) => {
                let mut display_string = String::from(format!(
                    "0,{},\"{}\"",
                    background.start_time, background.filename
                ));
                if let Some(x_offset) = background.x_offset {
                    display_string.push_str(&format!(",{}", x_offset));
                }
                if let Some(y_offset) = background.y_offset {
                    display_string.push_str(&format!(",{}", y_offset));
                }
                display_string.push_str("\n");
                write!(f, "{}", display_string)
            }
            Event::Video(video) => {
                let mut display_string =
                    String::from(format!("Video,{},\"{}\"", video.start_time, video.filename));
                if let Some(x_offset) = video.x_offset {
                    display_string.push_str(&format!(",{}", x_offset));
                }
                if let Some(y_offset) = video.y_offset {
                    display_string.push_str(&format!(",{}", y_offset));
                }

                display_string.push_str("\n");
                write!(f, "{}", display_string)
            }
            Event::Break(osu_break) => {
                write!(f, "2,{},{}\n", osu_break.start_time, osu_break.end_time)
            }
            Event::Storyboard(storyboard) => write!(f, "{}", storyboard),
            Event::Basic(osu_event) => {
                let mut params = String::new();
                if let Some(p) = &osu_event.params {
                    for (index, param) in p.iter().enumerate() {
                        if index == p.len() - 1 {
                            params.push_str(&param);
                        } else {
                            params.push_str(&format!("{},", param));
                        }
                    }
                }
                write!(
                    f,
                    "{},{},{}\n",
                    osu_event.event_type, osu_event.start_time, params
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Events {
    events: Vec<Event>,
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.events.is_empty() {
            return write!(f, "");
        }

        let mut display_string = String::from("[Events]\n");

        for event in &self.events {
            display_string.push_str(&format!("{}", event));
        }

        write!(f, "{}", display_string)
    }
}

impl Beatmap {
    pub fn get_background_path(&self) -> Option<String> {
        if let Some(background) = self.events.get_background() {
            return Some(
                OsString::from(PathBuf::from(self.get_directory()).join(background))
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
        None
    }

    pub fn get_video_path(&self) -> Option<String> {
        if let Some(video) = self.events.get_video() {
            return Some(
                OsString::from(PathBuf::from(self.get_directory()).join(video))
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }
        None
    }
}

impl Default for Events {
    fn default() -> Self {
        Events { events: Vec::new() }
    }
}

impl Events {
    pub fn parse_value(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
        match parts[0] {
            "0" => {
                let start_time = parts[1].parse().unwrap();
                let file_name = String::from(&parts[2][1..parts[2].len() - 1]);

                let x_offset = if parts.len() > 3 {
                    Some(parts[3].parse().unwrap())
                } else {
                    None
                };
                let y_offset = if parts.len() > 4 {
                    Some(parts[4].parse().unwrap())
                } else {
                    None
                };
                self.events.push(Event::Background(OsuBackground::new(
                    start_time, file_name, x_offset, y_offset,
                )));
            }
            "1" | "Video" => {
                let start_time = parts[1].parse().unwrap();
                let file_name = String::from(&parts[2][1..parts[2].len() - 1]);
                let x_offset = if parts.len() > 3 {
                    Some(parts[3].parse().unwrap())
                } else {
                    None
                };
                let y_offset = if parts.len() > 4 {
                    Some(parts[4].parse().unwrap())
                } else {
                    None
                };

                self.events.push(Event::Video(OsuVideo::new(
                    start_time, file_name, x_offset, y_offset,
                )));
            }
            "2" | "Break" => {
                let start_time = parts[1].parse().unwrap();
                let end_time = parts[2].parse().unwrap();

                self.events
                    .push(Event::Break(OsuBreak::new(start_time, end_time)));
            }
            _ => {
                if parts.len() > 2 {
                    let start_time = parts[1].parse().unwrap();
                    let mut params = Vec::new();
                    for p in parts[2..].iter() {
                        params.push(p.to_string());
                    }
                    self.events.push(Event::Basic(OsuEvent::new(
                        parts[0].to_string(),
                        start_time,
                        Some(params),
                    )));
                }
            }
        }
    }

    pub fn push_storyboard(&mut self, storyboard: OsuStoryboard) {
        self.events.push(Event::Storyboard(storyboard));
    }

    pub fn get_background(&self) -> Option<&str> {
        for event in &self.events {
            if let Event::Background(background) = event {
                return Some(&background.filename);
            }
        }
        None
    }

    pub fn get_video(&self) -> Option<&str> {
        for event in &self.events {
            if let Event::Video(video) = event {
                return Some(&video.filename);
            }
        }
        None
    }
}
