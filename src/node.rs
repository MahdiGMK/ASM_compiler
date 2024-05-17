use crate::command::{Command, UnableToParseError};
use std::str::FromStr;

#[derive(Debug)]
pub enum NodeType {
    State,
    Decision,
    Conditional,
}
#[derive(Debug)]
pub struct Node {
    node_name: String,
    node_type: NodeType,
    commands: Vec<Command>,
}
impl Node {
    pub fn try_parse(
        name: &str,
        node_type: &str,
        contents: &str,
    ) -> Result<Self, UnableToParseError> {
        let mut command_strs = contents.split(';');
        let node_type = match node_type.trim() {
            "state" => NodeType::State,
            "conditional" => NodeType::Conditional,
            "decision" => NodeType::Decision,
            _ => return Err(UnableToParseError::InvalidFormat),
        };
        let mut result = Node {
            node_name: name.to_string(),
            node_type,
            commands: vec![],
        };
        while let Some(str) = command_strs.next() {
            result.commands.push(str.parse()?)
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing_test() {
        let node = Node::try_parse(
            ".123salam",
            "state",
            "
r0   =>  1;
r1       =>  2;
r3    =>   r0 + r1;
then    =>    .state;
",
        );

        println!("{:?}", node);
    }
}
