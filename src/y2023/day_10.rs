use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pipe {
    Vertical(Direction, Direction),   // |
    Horizontal(Direction, Direction), // -
    NorthEast(Direction, Direction),  // L
    NorthWest(Direction, Direction),  // J
    SouthWest(Direction, Direction),  // 7
    SouthEast(Direction, Direction),  // F
    Ground,                           // .
    StartPosition,                    // S
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            '|' => Some(Pipe::Vertical(Direction::Top, Direction::Down)),
            '-' => Some(Pipe::Horizontal(Direction::Left, Direction::Right)),
            'L' => Some(Pipe::NorthEast(Direction::Top, Direction::Right)),
            'J' => Some(Pipe::NorthWest(Direction::Top, Direction::Left)),
            '7' => Some(Pipe::SouthWest(Direction::Down, Direction::Left)),
            'F' => Some(Pipe::SouthEast(Direction::Down, Direction::Right)),
            '.' => Some(Pipe::Ground),
            'S' => Some(Pipe::StartPosition),
            _ => None,
        }
    }

    fn get_directions(&self) -> (Direction, Direction) {
        match *self {
            Pipe::Vertical(Direction::Top, Direction::Down) => {
                (Direction::Top, Direction::Down)
            }
            Pipe::Horizontal(Direction::Left, Direction::Right) => {
                (Direction::Left, Direction::Right)
            }
            Pipe::NorthEast(Direction::Top, Direction::Right) => {
                (Direction::Top, Direction::Right)
            }
            Pipe::NorthWest(Direction::Top, Direction::Left) => {
                (Direction::Top, Direction::Left)
            }
            Pipe::SouthWest(Direction::Down, Direction::Left) => {
                (Direction::Down, Direction::Left)
            }
            Pipe::SouthEast(Direction::Down, Direction::Right) => {
                (Direction::Down, Direction::Right)
            }
            _ => (Direction::None, Direction::None),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn iter(&self) -> impl Iterator<Item = &Direction> {
        std::iter::once(self)
    }
}

impl Into<Pipe> for char {
    fn into(self) -> Pipe {
        Pipe::from_char(self).unwrap_or(Pipe::Ground)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub pipe: Pipe,
}

impl Point {
    pub fn new(x: usize, y: usize, pipe: Pipe) -> Self {
        Point { x, y, pipe }
    }

    pub fn step(&mut self, dx: i32, dy: i32, pipe: &Pipe) {
        self.x = (self.x as i32 + dx) as usize;
        self.y = (self.y as i32 + dy) as usize;
        self.pipe = pipe.clone();
    }

    pub fn is_connected_to(
        &self,
        other: &Point,
        direction: &Direction,
    ) -> bool {
        match (self.pipe, other.pipe, direction) {
            (Pipe::Horizontal(_, _), Pipe::Horizontal(_, _), Direction::Left)
            | (Pipe::Horizontal(_, _), Pipe::Horizontal(_, _), Direction::Right)
            | (Pipe::Horizontal(_, _), Pipe::NorthEast(_, _), Direction::Left)
            | (Pipe::Horizontal(_, _), Pipe::SouthEast(_, _), Direction::Left)
            | (Pipe::Horizontal(_, _), Pipe::NorthWest(_, _), Direction::Right)
            | (Pipe::Horizontal(_, _), Pipe::SouthWest(_, _), Direction::Right)

            | (Pipe::Vertical(_, _), Pipe::Vertical(_, _), Direction::Top)
            | (Pipe::Vertical(_, _), Pipe::Vertical(_, _), Direction::Down)
            | (Pipe::Vertical(_, _), Pipe::SouthWest(_, _), Direction::Top)
            | (Pipe::Vertical(_, _), Pipe::SouthEast(_, _), Direction::Top)
            | (Pipe::Vertical(_, _), Pipe::NorthWest(_, _), Direction::Down)
            | (Pipe::Vertical(_, _), Pipe::NorthEast(_, _), Direction::Down)

            | (Pipe::NorthEast(_, _), Pipe::Vertical(_, _), Direction::Top)
            | (Pipe::NorthEast(_, _), Pipe::SouthWest(_, _), Direction::Top)
            | (Pipe::NorthEast(_, _), Pipe::SouthEast(_, _), Direction::Top)
            | (Pipe::NorthEast(_, _), Pipe::Horizontal(_, _), Direction::Right)
            | (Pipe::NorthEast(_, _), Pipe::NorthWest(_, _), Direction::Right)
            | (Pipe::NorthEast(_, _), Pipe::SouthWest(_, _), Direction::Right)

            | (Pipe::NorthWest(_, _), Pipe::Vertical(_, _), Direction::Top)
            | (Pipe::NorthWest(_, _), Pipe::SouthWest(_, _), Direction::Top)
            | (Pipe::NorthWest(_, _), Pipe::SouthEast(_, _), Direction::Top)
            | (Pipe::NorthWest(_, _), Pipe::Horizontal(_, _), Direction::Left)
            | (Pipe::NorthWest(_, _), Pipe::SouthEast(_, _), Direction::Left)
            | (Pipe::NorthWest(_, _), Pipe::NorthEast(_, _), Direction::Left)

            | (Pipe::SouthWest(_, _), Pipe::Vertical(_, _), Direction::Down)
            | (Pipe::SouthWest(_, _), Pipe::NorthWest(_, _), Direction::Down)
            | (Pipe::SouthWest(_, _), Pipe::NorthEast(_, _), Direction::Down)
            | (Pipe::SouthWest(_, _), Pipe::Horizontal(_, _), Direction::Left)
            | (Pipe::SouthWest(_, _), Pipe::NorthEast(_, _), Direction::Left)
            | (Pipe::SouthWest(_, _), Pipe::SouthEast(_, _), Direction::Left)

            | (Pipe::SouthEast(_, _), Pipe::Vertical(_, _), Direction::Down)
            | (Pipe::SouthEast(_, _), Pipe::NorthWest(_, _), Direction::Down)
            | (Pipe::SouthEast(_, _), Pipe::NorthEast(_, _), Direction::Down)
            | (Pipe::SouthEast(_, _), Pipe::Horizontal(_, _), Direction::Right)
            | (Pipe::SouthEast(_, _), Pipe::NorthWest(_, _), Direction::Right)
            | (Pipe::SouthEast(_, _), Pipe::SouthWest(_, _), Direction::Right)
             => true,

            _ => false,
        }
    }

    pub fn is_start_point_connected_to(
        &self,
        other: &Point,
        direction: &Direction,
    ) -> bool {
        match (other.pipe, direction) {
            (Pipe::Vertical(_, _), Direction::Top)
            | (Pipe::SouthWest(_, _), Direction::Top)
            | (Pipe::SouthEast(_, _), Direction::Top)
            
            | (Pipe::Vertical(_, _), Direction::Down)
            | (Pipe::NorthWest(_, _), Direction::Down)
            | (Pipe::NorthEast(_, _), Direction::Down)

            | (Pipe::Horizontal(_, _), Direction::Left)
            | (Pipe::SouthEast(_, _), Direction::Left)
            | (Pipe::NorthEast(_, _), Direction::Left)

            | (Pipe::Horizontal(_, _), Direction::Right)
            | (Pipe::SouthWest(_, _), Direction::Right)
            | (Pipe::NorthWest(_, _), Direction::Right)
             => true,

            _ => false,
        }
    }
}

fn get_start_directions(
    matrix: &Vec<Vec<char>>,
    current_position: &Point,
) -> Vec<Direction> {
    let directions: Vec<Direction> = vec![
        Direction::Top,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    let mut directions_to_start: Vec<Direction> = Vec::new();

    for direction in directions.iter() {
        let mut is_connected = false;
        match *direction {
            Direction::Top => {
                if current_position.y > 0 {
                    // top exists
                    let other_point = Point::new(
                        current_position.x,
                        current_position.y - 1,
                        matrix[current_position.y - 1][current_position.x]
                            .into(),
                    );
                    is_connected = current_position
                        .is_start_point_connected_to(&other_point, direction);
                    if is_connected {
                        directions_to_start.push(*direction);
                    }
                }
            }
            Direction::Down => {
                if current_position.y < matrix[0].len() - 1 {
                    // down exists
                    let other_point = Point::new(
                        current_position.x,
                        current_position.y + 1,
                        matrix[current_position.y + 1][current_position.x]
                            .into(),
                    );
                    is_connected = current_position
                        .is_start_point_connected_to(&other_point, direction);
                    if is_connected {
                        directions_to_start.push(*direction);
                    }
                }
            }
            Direction::Left => {
                if current_position.x > 0 {
                    // left exists
                    let other_point = Point::new(
                        current_position.x - 1,
                        current_position.y,
                        matrix[current_position.y][current_position.x - 1]
                            .into(),
                    );
                    is_connected = current_position
                        .is_start_point_connected_to(&other_point, direction);
                    if is_connected {
                        directions_to_start.push(*direction);
                    }
                }
            }
            Direction::Right => {
                if current_position.x < matrix.len() - 1 {
                    // right exists
                    let other_point = Point::new(
                        current_position.x + 1,
                        current_position.y,
                        matrix[current_position.y][current_position.x + 1]
                            .into(),
                    );
                    is_connected = current_position
                        .is_start_point_connected_to(&other_point, direction);
                    if is_connected {
                        directions_to_start.push(*direction);
                    }
                }
            }
            _ => (),
        }
    }

    directions_to_start
}

fn traverse(
    current_position: &mut Point,
    matrix: &Vec<Vec<char>>,
    direction: &Direction,
) {
    let (mut dx, mut dy) = (0, 0);
    match direction {
        Direction::Top => {
            dy = -1;
        }
        Direction::Right => {
            dx = 1;
        }
        Direction::Down => {
            dy = 1;
        }
        Direction::Left => {
            dx = -1;
        }
        _ => (),
    }

    let next_pipe = matrix[(current_position.y as i32 + dy) as usize]
        [(current_position.x as i32 + dx) as usize]
        .into();
    current_position.step(dx, dy, &next_pipe);
}

fn peek_next_point(
    current_position: &Point,
    last_position: &Point,
    direction: &Direction,
) -> bool {
    let (mut dx, mut dy) = (0, 0);
    match direction {
        Direction::Top => {
            dy = -1;
        }
        Direction::Right => {
            dx = 1;
        }
        Direction::Down => {
            dy = 1;
        }
        Direction::Left => {
            dx = -1;
        }
        _ => (),
    }

    (current_position.y as i32 + dy) as usize == last_position.y
        && (current_position.x as i32 + dx) as usize == last_position.x
}

fn explore_maze(matrix: &Vec<Vec<char>>, start_coordinate: (usize, usize)) -> i32 {
    let mut current_position =
        Point::new(start_coordinate.0, start_coordinate.1, Pipe::StartPosition);
    let mut last_position: Point;
    let mut count = 0;

    let start_directions = get_start_directions(matrix, &current_position);
    println!("start_directions: {:?}", start_directions);

    let mut found = false;
    for start_direction in start_directions.iter() {
        // first traversal
        last_position = current_position.clone();
        traverse(&mut current_position, &matrix, start_direction);
        count += 1;

        loop {
            let next_pipe = current_position.pipe.clone();
            let new_directions: (Direction, Direction) =
                next_pipe.get_directions();
            println!("next_pipe: {:?}", next_pipe);

            for direction in
                new_directions.0.iter().chain(new_directions.1.iter())
            {
                let is_next_point_ok = peek_next_point(
                    &current_position,
                    &last_position,
                    direction,
                );
                if !is_next_point_ok {
                    println!("go this direction next {:?}", direction);
                    last_position = current_position.clone();
                    traverse(&mut current_position, &matrix, direction);
                    count += 1;
                    break;
                }
            }

            if current_position.x == start_coordinate.0
                && current_position.y == start_coordinate.1
            {
                println!("reached start position");
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    count / 2 
}

pub fn process_file(file_path: &str) -> i32 {
    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        // Initialize a vector to store vectors of characters (lines)
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let mut start_coordinate: Option<(usize, usize)> = None;

        for (row, line) in reader.lines().enumerate() {
            if let Ok(line_content) = line {
                let char_vector: Vec<char> = line_content.chars().collect();
                matrix.push(char_vector);

                // Check for 'S' and store its coordinate
                if let Some(col) = matrix[row].iter().position(|&c| c == 'S') {
                    start_coordinate = Some((row, col));
                }
            }
        }

        for row in matrix.iter() {
            println!("{:?}", row);
        }

        if let Some((row, col)) = start_coordinate {
            println!("Start coordinate: ({}, {})", col, row);

            // Explore the maze starting from the given coordinate
            explore_maze(&matrix, (col, row))
        } else {
            println!("No 'S' found in the matrix.");
            0
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
        0
    }
}
