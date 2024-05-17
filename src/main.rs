mod command;
mod node;
mod verilog_code_gen;
use command::{Command, UnableToParseError};
use node::Node;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::Range,
    path::Path,
    str::FromStr,
};
use verilog_code_gen::*;

use crate::node::NodeType;

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

    let mut code = Code {
        code: String::new(),
        hsh: 1231332,
    };
    let mut params = vec![];
    for cmd in top_level_commands.iter() {
        match cmd {
            Command::Input {
                pin_name,
                bits,
                array,
            } => {
                if array.start != array.end || array.start != 0 {
                    params.push(format!(
                        "input [{}:{}]{}[{}:{}]",
                        bits.start, bits.end, pin_name, array.start, array.end
                    ));
                } else {
                    params.push(format!("input [{}:{}]{}", bits.start, bits.end, pin_name));
                }
            }
            Command::Output {
                pin_name,
                bits,
                array,
            } => {
                if array.start != array.end || array.start != 0 {
                    params.push(format!(
                        "output reg [{}:{}]{}[{}:{}]",
                        bits.start, bits.end, pin_name, array.start, array.end
                    ));
                } else {
                    params.push(format!(
                        "output reg [{}:{}]{}",
                        bits.start, bits.end, pin_name
                    ));
                }
            }
            Command::Inout {
                pin_name,
                bits,
                array,
            } => {
                if array.start != array.end || array.start != 0 {
                    params.push(format!(
                        "inout [{}:{}]{}[{}:{}]",
                        bits.start, bits.end, pin_name, array.start, array.end
                    ));
                } else {
                    params.push(format!("inout [{}:{}]{}", bits.start, bits.end, pin_name));
                }
            }
            _ => {}
        }
    }

    code.update(format!(
        "
module {}(input clk , {});",
        module,
        params.join(" , ")
    ));

    for cmd in top_level_commands.iter() {
        match cmd {
            Command::Register {
                reg_name,
                bits,
                array,
            } => {
                if array.start != array.end || array.start != 0 {
                    params.push(format!(
                        "
reg [{}:{}]{}[{}:{}];",
                        bits.start, bits.end, reg_name, array.start, array.end
                    ));
                } else {
                    params.push(format!(
                        "
reg [{}:{}]{};",
                        bits.start, bits.end, reg_name
                    ));
                }
            }
            _ => {}
        }
    }

    let mut state_nodes = vec![];
    let mut node_map = HashMap::new();
    for node in all_nodes.iter_mut() {
        if node.node_type == NodeType::State {
            node.id = state_nodes.len() as u32;
            state_nodes.push(node);
        }
        node_map.insert(&node.node_name, node);
    }

    let bit_count = state_nodes.len().ilog2() as u8;
    let current_state_reg = code.get_varname(&"currentState".to_string());
    code.update(format!(
        "
reg [{bit_count}:0]{current_state_reg} = 0;",
    ));

    code.update(
        "
always(*) begin"
            .to_string(),
    );

    for node in state_nodes.iter() {
        let mut seen = HashSet::new();
        if !compile_node(&mut code, node, &node_map, &mut seen) {
            return Err(UnableToParseError::CircularDependency);
        }
    }

    code.update(
        "
end"
        .to_string(),
    );

    code.update(
        "
endmodule"
            .to_string(),
    );

    println!("{}", code.code);

    Ok(())
}

fn compile_node<'l>(
    code: &mut Code,
    node: &'l Node,
    node_map: &'l HashMap<&'l String, &'l mut Node>,
    seen: &mut HashSet<&'l String>,
) -> bool {
    if seen.contains(&node.node_name) {
        return false;
    }
    seen.insert(&node.node_name);
    if node.node_type == NodeType::Decision {
        let mut check_cond = "0".to_string();
        let mut yes_node = "".to_string();
        let mut no_node = "".to_string();
        for command in node.commands.iter() {
            match command {
                Command::Check { check } => {
                    check_cond = check.to_string();
                }
                Command::Yes { next_node } => {
                    yes_node = next_node.to_string();
                }
                Command::No { next_node } => {
                    no_node = next_node.to_string();
                }
                _ => {}
            }
        }

        code.update(format!(
            "
if ({}) begin",
            check_cond
        ));

        if !compile_node(code, node_map.get(&yes_node).unwrap(), node_map, seen) {
            return false;
        }

        code.update(format!(
            "
end else begin"
        ));

        if !compile_node(code, node_map.get(&no_node).unwrap(), node_map, seen) {
            return false;
        }

        code.update(format!(
            "
end"
        ));
    }
    let mut then_node = "".to_string();
    for command in node.commands.iter() {
        match command {
            Command::RegisterTransfer {
                reg_name,
                reg_value,
            } => {
                code.update(format!(
                    "
{} <= {};",
                    reg_name, reg_value
                ));
            }
            Command::Then { next_node } => {
                then_node = next_node.to_string();
            }
            _ => {}
        }
    }

    compile_node(code, node_map.get(&then_node).unwrap(), node_map, seen)
}
