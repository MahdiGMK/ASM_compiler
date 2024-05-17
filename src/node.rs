use crate::command::{Command, UnableToParseError};
use std::str::FromStr;

pub struct Node {
    commands: Vec<Command>,
}
impl FromStr for Node {
    type Err = UnableToParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut command_strs = s.split(';');
        let mut result = Node { commands: vec![] };
        while let Some(str) = command_strs.next() {
            result.commands.push(str.parse()?)
        }
        Ok(result)
    }
}
