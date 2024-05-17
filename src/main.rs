mod command;
mod node;
mod verilog_code_gen;
use command::{Command, UnableToParseError};
use node::Node;
use regex::Regex;
use std::{fs::read_to_string, ops::Range, path::Path, str::FromStr};
use verilog_code_gen::*;

fn main() -> Result<(), UnableToParseError> {
    let mut all_args = std::env::args();
    let file_path = all_args.nth(1).expect("no file given");
    let mut contents = read_to_string(Path::new(&file_path))
        .expect("unable to read file")
        .to_string();
    contents.push_str(
        "
.random_default_node : state{}
",
    );

    let mut outpath = "output.v".to_string();
    let mut module = "ArrayMultiplier".to_string();
    while let Some(flag_name) = all_args.next() {
        match flag_name.as_ref() {
            "-o" | "--output" => {
                if let Some(path_dir) = all_args.next() {
                    outpath = path_dir;
                }
            }
            "-n" | "--name" => {
                if let Some(mod_name) = all_args.next() {
                    module = mod_name;
                }
            }
            _ => {}
        }
    }

    let node_regex =
        Regex::new(r"([^}]*)\.([a-zA-Z0-9_]+) *: *(state|decision|conditional) *\{([^.]*)}([^.]*)")
            .unwrap();

    let mut all_nodes: Vec<node::Node> = vec![];
    let mut top_level_commands: Vec<Command> = vec![];

    for capt in node_regex.captures_iter(&contents) {
        println!("{}", capt.get(0).unwrap().as_str());
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

        for cmd in pre_node.as_str().split(';') {
            let cmd: Command = cmd.parse()?;
            if cmd != Command::Empty {
                top_level_commands.push(cmd);
            }
        }
        for cmd in post_node.as_str().split(';') {
            let cmd: Command = cmd.parse()?;
            if cmd != Command::Empty {
                top_level_commands.push(cmd);
            }
        }
    }
    all_nodes.pop();

    let mut fullCode = Code {
        code: String::new(),
        hsh: 1231332,
    };
    let mut params = vec![];
    for cmd in top_level_commands {
        match cmd {
            Command::Input {
                pin_name,
                bits,
                array,
            } => {
                params.push(format!(
                    "input [{}:{}]{}[{}:{}]",
                    bits.start, bits.end, pin_name, array.start, array.end
                ));
            }
            Command::Output {
                pin_name,
                bits,
                array,
            } => {
                params.push(format!(
                    "output reg [{}:{}]{}[{}:{}]",
                    bits.start, bits.end, pin_name, array.start, array.end
                ));
            }
            Command::Inout {
                pin_name,
                bits,
                array,
            } => {
                params.push(format!(
                    "inout [{}:{}]{}[{}:{}]",
                    bits.start, bits.end, pin_name, array.start, array.end
                ));
            }
            _ => {}
        }
    }

    Ok(())
}
