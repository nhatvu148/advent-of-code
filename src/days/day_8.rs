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

fn contains(graph: &BTreeMap<String, Vec<String>>, s: impl ToString) -> bool {
    let target = s.to_string();

    for neighbors in graph.values() {
        for node in neighbors {
            if node.ends_with(&target) {
                return true;
            }
        }
    }
    false
}

pub fn traverse_graph(
    graph: &BTreeMap<String, Vec<String>>,
    instructions: &str,
) -> u64 {
    if !contains(&graph, "ZZZ") {
        return 0;
    }

    let start_node = graph.keys().next().map(|s| s.clone()).unwrap_or_default();
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
        }
    }

    steps
}

fn find_start_nodes(graph: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    graph
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect()
}

fn traverse_graph_ends_with_z(
    graph: &BTreeMap<String, Vec<String>>,
    start_node: &str,
    instructions: &str,
) -> u64 {
    if !contains(&graph, 'Z') {
        return 0;
    }

    let mut current_node = start_node.to_string();
    let mut steps = 0;

    while !current_node.ends_with('Z') {
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
        }
    }

    steps
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

const fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}

pub fn find_steps_lcm(
    graph: &BTreeMap<String, Vec<String>>,
    instructions: &str,
) -> u64 {
    let start_nodes = find_start_nodes(&graph);
    let step_vector: Vec<u64> = start_nodes
        .iter()
        .map(|current_node| {
            traverse_graph_ends_with_z(&graph, &current_node, &instructions)
        })
        .collect();

    step_vector.clone().into_iter().reduce(lcm).unwrap()
}

fn _traverse_graph_2(
    graph: &BTreeMap<String, Vec<String>>,
    instructions: &str,
) -> u32 {
    let mut current_nodes = find_start_nodes(&graph);
    let mut steps = 0;

    while !current_nodes.iter().all(|node| node.ends_with('Z')) {
        for instruction in instructions.chars() {
            let next_nodes: Vec<String> = current_nodes
                .iter()
                .flat_map(|current_node| {
                    match instruction {
                        'R' => graph
                            .get(current_node)
                            .and_then(|neighbors| neighbors.get(1)),
                        'L' => graph
                            .get(current_node)
                            .and_then(|neighbors| neighbors.get(0)),
                        _ => None,
                    }
                    .map(|node| node.to_string())
                })
                .collect();

            if next_nodes.len() == current_nodes.len() {
                println!("next_nodes: {:?}", &next_nodes);
                current_nodes = next_nodes;
                steps += 1;
            } else {
                // Break the loop if the condition is not met
                break;
            }
        }
    }

    steps
}
