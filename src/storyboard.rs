use std::{io::Read, path::Path};

use crate::{
    sanitize::sanitize,
    section::{Command, OsuStoryboard},
};

pub struct Storyboard {
    file_name: String,
    pub storyboards: Vec<OsuStoryboard>,
}

impl std::fmt::Display for Storyboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.storyboards.is_empty() {
            return write!(f, "");
        }

        let mut display_string = String::from("[Events]\n");

        for storyboard in &self.storyboards {
            display_string += &format!("{}", storyboard);
        }

        write!(f, "{}", display_string)
    }
}

impl Storyboard {
    pub fn new(file_name: &str) -> Self {
        Storyboard {
            file_name: file_name.to_string(),
            storyboards: Vec::new(),
        }
    }

    pub fn push_storyboard(&mut self, storyboard: OsuStoryboard) {
        self.storyboards.push(storyboard);
    }

    pub fn parse(osb_data: &str, file_name: &str) -> std::io::Result<Storyboard> {
        let mut storyboard = Storyboard::new(file_name);

        let sanitized = sanitize(osb_data);

        let lines = sanitized
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();

        let mut index = 0;

        if let Some(line) = lines.get(index) {
            if !line.contains("[Events]") {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid Storyboard file",
                ));
            } else {
                index += 1;

                while let Some(line) = lines.get(index) {
                    if let Ok(mut osu_storyboard) = OsuStoryboard::parse(line.as_str()) {
                        index += 1;
                        'command_loop: while let Some(potential_command) = lines.get(index) {
                            if let Ok(command) = Command::parse(&potential_command) {
                                osu_storyboard.add_command(command);

                                index += 1;
                            } else {
                                break 'command_loop;
                            }
                        }
                        storyboard.push_storyboard(osu_storyboard);
                    }
                }
            }
        }

        Ok(storyboard)
    }

    pub fn parse_file(file: &str) -> std::io::Result<Storyboard> {
        let path = Path::new(file);
        let name = path.file_name().unwrap().to_str().unwrap();
        let mut file = std::fs::File::open(path)?;

        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        Storyboard::parse(&contents, name)
    }
    pub fn save_with_name(&self, name: &str) {
        std::fs::write(name, self.to_string()).unwrap();
    }

    pub fn save(&self) {
        self.save_with_name(self.get_file_name());
    }

    pub fn get_file_name(&self) -> &str {
        &self.file_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storyboard_parse() {
        let storyboard = Storyboard::parse_file("storyboard.osb").unwrap();
        storyboard.save_with_name("test_storyboard.osb");
    }
}
