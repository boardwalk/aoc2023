use anyhow::{anyhow, Error};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]

struct Node {
    left: String,
    right: String,
}

fn find_num_steps(instructions: &str, nodes: &HashMap<String, Node>) -> usize {
    let mut cur_pos = "AAA";

    let mut num_steps = 0;

    let mut instr_iter = instructions.chars().cycle();

    while cur_pos != "ZZZ" {
        let instr = instr_iter.next().unwrap();
        println!("looking up {cur_pos}");
        let cur_node = nodes.get(cur_pos).unwrap();
        match instr {
            'L' => {
                cur_pos = cur_node.left.as_str();
            }
            'R' => {
                cur_pos = cur_node.right.as_str();
            }
            _ => panic!("invalid instruction"),
        }
        num_steps += 1;
    }

    num_steps
}
fn main() -> Result<(), Error> {
    let mut instructions = None;

    let mut nodes: HashMap<String, Node> = HashMap::new();

    let node_re = Regex::new(r#"^([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)$"#)?;

    for (line_no, line) in std::io::stdin().lines().enumerate() {
        let line = line?;

        if line_no == 0 {
            // instructions line
            instructions = Some(line);
        } else if line_no == 1 {
            // blank line
        } else {
            // node line
            let captures = node_re
                .captures(&line)
                .ok_or_else(|| anyhow!("line did not match expected format"))?;

            let (_, [src, left, right]) = captures.extract();

            nodes.insert(
                src.to_owned(),
                Node {
                    left: left.to_owned(),
                    right: right.to_owned(),
                },
            );
        }
    }

    let instructions = instructions.ok_or_else(|| anyhow!("too few lines in input"))?;

    println!("{nodes:?}");

    let num_steps = find_num_steps(&instructions, &nodes);

    println!("{num_steps}");
    Ok(())
}
