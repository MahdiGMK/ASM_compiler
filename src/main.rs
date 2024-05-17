mod command;
mod node;
use command::Command;
use node::Node;
use regex::Regex;
use std::{fs::read_to_string, ops::Range, path::Path, str::FromStr};

fn main() {
    let mut all_args = std::env::args();
    let file_path = all_args.nth(1).expect("no file given");
    let contents = read_to_string(Path::new(&file_path)).expect("unable to read file");

    let node_regex = Regex::new(r"(.[a-zA-Z0-9_]+) *\{(.*)}").unwrap();

    for capt in node_regex.captures_iter(&contents) {}
}
