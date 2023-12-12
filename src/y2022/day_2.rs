use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
pub enum HandShape {
    A, // Rock (Opponent)
    B, // Paper (Opponent)
    C, // Scissors (Opponent)
    X, // Rock (Player)
    Y, // Paper (Player)
    Z, // Scissors (Player)
    None,
}

impl HandShape {
    pub fn from_char(c: char) -> Option<HandShape> {
        match c {
            'A' => Some(HandShape::A),
            'B' => Some(HandShape::B),
            'C' => Some(HandShape::C),
            'X' => Some(HandShape::X),
            'Y' => Some(HandShape::Y),
            'Z' => Some(HandShape::Z),
            _ => None,
        }
    }

    pub fn compare(&self, other: &HandShape) -> i32 {
        match (self, other) {
            (HandShape::A, HandShape::X)
            | (HandShape::B, HandShape::Y)
            | (HandShape::C, HandShape::Z) => 3, // Draw
            (HandShape::A, HandShape::Y)
            | (HandShape::B, HandShape::Z)
            | (HandShape::C, HandShape::X) => 6, // Win
            _ => 0, // Lose
        }
    }

    pub fn get_score(&self) -> i32 {
        match self {
            HandShape::A | HandShape::X => 1,
            HandShape::B | HandShape::Y => 2,
            HandShape::C | HandShape::Z => 3,
            _ => 0,
        }
    }
}

impl Into<HandShape> for char {
    fn into(self) -> HandShape {
        HandShape::from_char(self).unwrap_or(HandShape::None)
    }
}

pub fn process_file(file_path: &str) -> i32 {
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        // Process each line in the file
        let mut total_score = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            println!("line: {:?}", line);
            let mut iter = line.split_whitespace();

            while let (Some(opponent_choice), Some(player_choice)) =
                (iter.next(), iter.next())
            {
                // Convert each substring to &char
                if let (Some(opponent_char), Some(player_char)) = (
                    opponent_choice.chars().next(),
                    player_choice.chars().next(),
                ) {
                    let opponent_shape = HandShape::from_char(opponent_char);
                    let player_shape = HandShape::from_char(player_char);

                    if let (Some(opponent_shape), Some(player_shape)) =
                        (opponent_shape, player_shape)
                    {
                        println!(
                            "opponent_choice: {:?}, player_choice: {:?}",
                            opponent_shape, player_shape
                        );
                        let result = opponent_shape.compare(&player_shape);
                        let score = result + player_shape.get_score();
                        total_score += score;
                        println!("score: {}", score);

                        // match result {
                        //     0 => println!("You lose!"),
                        //     3 => println!("It's a draw!"),
                        //     6 => println!("You win!"),
                        //     _ => println!("Invalid choices."),
                        // }
                    } else {
                        println!("Invalid choices.");
                    }
                }
            }
        }

        println!("total_score: {}", total_score);
        total_score
    } else {
        eprintln!("Error opening file: {}", file_path);
        0
    }
}
