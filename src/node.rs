use crate::command::{Command, UnableToParseError};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    State,
    Decision,
    Conditional,
}
#[derive(Debug)]
pub struct Node {
    pub node_name: String,
    pub node_type: NodeType,
    pub commands: Vec<Command>,
    pub id: u32,
}
impl Node {
    pub fn get_name(&self) -> String {
        self.node_name.clone()
    }
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
            id: 0,
            node_name: name.to_string(),
            node_type,
            commands: vec![],
        };
        while let Some(str) = command_strs.next() {
            let cmd: Command = str.parse()?;
            if cmd != Command::Empty {
                result.commands.push(cmd);
            }
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

then    =>    .state;
",
        );
        match node {
            Ok(Node {
                node_name,
                node_type,
                commands,
            }) => {
                assert_eq!(node_name, ".123salam".to_string());
                assert_eq!(node_type, NodeType::State);
                assert_eq!(commands.len(), 3);
                assert_eq!(
                    commands[0],
                    Command::RegisterTransfer {
                        reg_name: "r0".to_string(),
                        reg_value: "1".to_string()
                    }
                );
                assert_eq!(
                    commands[1],
                    Command::Then {
                        next_node: ".state".to_string()
                    }
                );
                assert_eq!(commands[2], Command::Empty);
            }
            _ => assert!(false),
        }
    }
}
