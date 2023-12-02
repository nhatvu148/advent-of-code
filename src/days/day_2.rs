use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[cfg(test)]
#[path = "../unit_tests/day_2.rs"]
mod day_2_tests;

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub subsets: Vec<Subset>,
}

#[derive(Debug)]
pub struct Subset {
    pub counts: HashMap<String, usize>,
}

pub type CubeCounts = std::collections::HashMap<String, usize>;

pub fn read_games_from_file(file_path: &str) -> Result<Vec<Game>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((id, subsets)) = parse_game_line(&line) {
            games.push(Game { id, subsets });
        }
    }

    Ok(games)
}

fn parse_game_line(line: &str) -> Option<(usize, Vec<Subset>)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() == 2 {
        if let Ok(id) = parts[0]
            .trim()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
        {
            let subsets = parts[1]
                .split(';')
                .flat_map(|subset_str| parse_subset(subset_str))
                .collect();
            return Some((id, subsets));
        }
    }
    None
}

fn parse_subset(subset_str: &str) -> Option<Subset> {
    let mut counts = HashMap::new();
    let parts: Vec<&str> = subset_str.split(',').map(|s| s.trim()).collect();

    for part in parts.iter() {
        let p: Vec<&str> = part.split_whitespace().collect();
        if let Ok(count) = p[0].parse::<usize>() {
            let color = p[1..].join(" ").to_lowercase();
            counts.insert(color, count);
        }
    }
    Some(Subset { counts })
}

impl Game {
    pub fn is_possible(&self, counts: &HashMap<String, usize>) -> bool {
        for subset in &self.subsets {
            for (color, count) in &subset.counts {
                if let Some(&available_count) = counts.get(color) {
                    if *count > available_count {
                        return false;
                    }
                } else {
                    return false; // Color not present in counts
                }
            }
        }
        true
    }

    pub fn calculate_max_set(&self) -> Option<Subset> {
        let mut counts = HashMap::new();

        for subset in &self.subsets {
            for (color, count) in &subset.counts {
                match counts.get(color.as_str()) {
                    Some(&prev) => {
                        if prev < *count {
                            counts.insert(color.clone(), *count);
                        }
                    }
                    None => {
                        counts.insert(color.clone(), *count);
                    }
                }
            }
        }

        Some(Subset { counts })
    }
}

pub fn is_game_possible(game: &Game, counts: &HashMap<String, usize>) -> bool {
    game.is_possible(counts)
}

pub fn calculate_power_max_set(games: &[Game]) -> usize {
    let mut total_max_set = 0;

    for game in games {
        let mut product = 1;
        let max_set = game.calculate_max_set().unwrap();
        for (_key, value) in max_set.counts.iter() {
            product *= value;
        }
        total_max_set += product;
    }

    total_max_set
}
