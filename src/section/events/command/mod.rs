mod color_command;
mod fade_command;
mod loop_command;
mod move_command;
mod move_x_command;
mod move_y_command;
mod parameter_command;
mod rotate_command;
mod scale_command;
mod trigger_command;
mod vector_scale_command;

use color_command::ColorCommand;
use fade_command::FadeCommand;
use loop_command::LoopCommand;
use move_command::MoveCommand;
use move_x_command::MoveXCommand;
use move_y_command::MoveYCommand;
use parameter_command::ParameterCommand;
use rotate_command::RotateCommand;
use scale_command::ScaleCommand;
use trigger_command::TriggerCommand;
use vector_scale_command::VectorScaleCommand;

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
