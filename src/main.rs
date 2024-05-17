mod command;
mod node;
use command::{Command, UnableToParseError};
use node::Node;
use regex::Regex;
use std::{fs::read_to_string, ops::Range, path::Path, str::FromStr};

fn main() -> Result<(), UnableToParseError> {
    let mut all_args = std::env::args();
    let file_path = all_args.nth(1).expect("no file given");
    let mut contents = read_to_string(Path::new(&file_path))
        .expect("unable to read file")
        .to_string();
    contents.push_str(
        "
.random_default_node{}
",
    );

    let node_regex =
        Regex::new(r"([^}]*)(\.[a-zA-Z0-9_]+) *: *(state|decision|conditional) *\{([^.]*)}([^.]*)")
            .unwrap();

    let mut all_nodes: Vec<node::Node> = vec![];
    let mut top_level_commands: Vec<Command> = vec![];

    for capt in node_regex.captures_iter(&contents) {
        let pre_node = capt.get(1).unwrap();
        let node_name = capt.get(2).unwrap();
        let node_type = capt.get(3).unwrap();
        let node_content = capt.get(4).unwrap();
        let post_node = capt.get(5).unwrap();
        all_nodes.push(Node::try_parse(
            node_name.as_str(),
            node_type.as_str(),
            node_content.as_str(),
        )?);

        for cmd_candi in pre_node.as_str().split(';') {
            top_level_commands.push(cmd_candi.parse()?);
        }
        for cmd_candi in post_node.as_str().split(';') {
            top_level_commands.push(cmd_candi.parse()?);
        }
    }
    Ok(())
}
