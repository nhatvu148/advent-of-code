use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};

pub struct GraphAndInstructions {
    pub graph: BTreeMap<String, Vec<String>>,
    pub instructions: String,
}

pub fn process_file(
    file_path: &str,
) -> Result<GraphAndInstructions, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut graph: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut instructions = String::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index == 0 {
            instructions = line.trim().to_string();
        } else {
            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();

            if parts.len() == 2 {
                let node = parts[0].to_string();
                let neighbors: Vec<String> = parts[1]
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                graph.insert(node, neighbors);
            }
        }
    }

    Ok(GraphAndInstructions {
        graph,
        instructions,
    })
}

pub fn traverse_graph(
    graph: &BTreeMap<String, Vec<String>>,
    start_node: &str,
    instructions: &str,
) -> u32 {
    let mut current_node = start_node.to_string();
    let mut steps = 0;

    while !current_node.eq_ignore_ascii_case("ZZZ") {
        for instruction in instructions.chars() {
            match instruction {
                'R' => {
                    if let Some(neighbors) = graph.get(&current_node) {
                        if let Some(right_node) = neighbors.get(1) {
                            current_node = right_node.to_string();
                            steps += 1;
                        }
                    }
                }
                'L' => {
                    if let Some(neighbors) = graph.get(&current_node) {
                        if let Some(left_node) = neighbors.get(0) {
                            current_node = left_node.to_string();
                            steps += 1;
                        }
                    }
                }
                _ => {
                    // Ignore unknown instructions
                }
            }

            if current_node.eq_ignore_ascii_case("ZZZ") {
                return steps;
            }
        }
    }

    0 // Return 0 if "ZZZ" is not reached
}
