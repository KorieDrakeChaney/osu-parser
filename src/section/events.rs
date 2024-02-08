use std::ffi::OsString;

use crate::token::EventsToken;
#[derive(Debug)]
struct Event {
    event_type: String,
    start_time: i32,
    params: String,
}
#[derive(Debug)]
struct OsuBackground {
    filename: OsString,
    x_offset: i32,
    y_offset: i32,
}
#[derive(Debug)]
struct OsuVideo {
    start_time: i32,
    filename: OsString,
    x_offset: i32,
    y_offset: i32,
}
#[derive(Debug)]
struct OsuBreak {
    start_time: i32,
    end_time: i32,
}
#[derive(Debug)]
pub struct Events {
    background: Option<OsuBackground>,
    video: Option<OsuVideo>,
    breaks: Vec<OsuBreak>,
    events: Vec<Event>,
}

impl From<&Vec<EventsToken>> for Events {
    fn from(tokens: &Vec<EventsToken>) -> Self {
        let mut background = None;
        let mut video = None;
        let mut breaks = Vec::new();
        let mut events = Vec::new();

        for token in tokens {
            match token {
                EventsToken::Background(filename, x_offset, y_offset) => {
                    background = Some(OsuBackground {
                        filename: OsString::from(filename),
                        x_offset: *x_offset,
                        y_offset: *y_offset,
                    });
                }
                EventsToken::Video(start_time, filename, x_offset, y_offset) => {
                    video = Some(OsuVideo {
                        start_time: *start_time,
                        filename: OsString::from(filename),
                        x_offset: *x_offset,
                        y_offset: *y_offset,
                    });
                }
                EventsToken::Break(start_time, end_time) => {
                    breaks.push(OsuBreak {
                        start_time: *start_time,
                        end_time: *end_time,
                    });
                }
                EventsToken::Event(event_type, start_time, params) => {
                    events.push(Event {
                        event_type: event_type.to_string(),
                        start_time: *start_time,
                        params: params.to_string(),
                    });
                }
                _ => {}
            }
        }

        Events {
            background,
            video,
            breaks,
            events,
        }
    }
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
        for event in &self.events {
            write!(
                f,
                "{},{},{}\n",
                event.event_type, event.start_time, event.params
            )?;
        }
        Ok(())
    }
}
