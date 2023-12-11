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
            | (Pipe::NorthWest(_, _), Direction::Right) => true,

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
) -> (f64, f64) {
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

    let new_x = (current_position.x as i32 + dx) as usize;
    let new_y = (current_position.y as i32 + dy) as usize;

    let next_pipe = matrix[new_y][new_x].into();
    current_position.step(dx, dy, &next_pipe);

    (new_x as f64, new_y as f64)
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

fn is_point_in_polygon(point: &(f64, f64), polygon: &Vec<(f64, f64)>) -> bool {
    // ray tracing
    let mut count = 0;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let pi = &polygon[i];
        let pj = &polygon[j];

        if (pi.1 > point.1) != (pj.1 > point.1)
            && point.0 < (pj.0 - pi.0) * (point.1 - pi.1) / (pj.1 - pi.1) + pi.0
        {
            count += 1;
        }
    }

    count % 2 == 1
}

fn explore_maze(
    matrix: &Vec<Vec<char>>,
    start_coordinate: (usize, usize),
) -> (i32, i32) {
    let mut current_position =
        Point::new(start_coordinate.0, start_coordinate.1, Pipe::StartPosition);
    let mut last_position: Point;
    let mut count = 0;
    let mut enclosed_count = 0;
    let mut polygon: Vec<(f64, f64)> =
        vec![(current_position.x as f64, current_position.y as f64)];

    let start_directions = get_start_directions(matrix, &current_position);
    // println!("start_directions: {:?}", start_directions);

    let mut found = false;
    for start_direction in start_directions.iter() {
        // first traversal
        last_position = current_position.clone();
        let new_coordinate =
            traverse(&mut current_position, &matrix, start_direction);
        polygon.push(new_coordinate);
        count += 1;

        loop {
            let next_pipe = current_position.pipe.clone();
            let new_directions: (Direction, Direction) =
                next_pipe.get_directions();
            // println!("next_pipe: {:?}", next_pipe);

            for direction in
                new_directions.0.iter().chain(new_directions.1.iter())
            {
                let is_next_point_ok = peek_next_point(
                    &current_position,
                    &last_position,
                    direction,
                );
                if !is_next_point_ok {
                    // println!("go this direction next {:?}", direction);
                    last_position = current_position.clone();
                    let new_coordinate =
                        traverse(&mut current_position, &matrix, direction);
                    polygon.push(new_coordinate);
                    count += 1;
                    break;
                }
            }

            if current_position.x == start_coordinate.0
                && current_position.y == start_coordinate.1
            {
                // println!("reached start position");
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if !polygon.contains(&(x as f64, y as f64))
                && is_point_in_polygon(&(x as f64, y as f64), &polygon)
            {
                enclosed_count += 1;
                // println!("enclosed polygon: {:?}", &(x as f64, y as f64));
            }
        }
    }

    // println!("polygon: {:?}", polygon);
    // println!("enclosed_count: {}", enclosed_count);

    (count / 2, enclosed_count)
}

pub fn process_file(file_path: &str) -> (i32, i32) {
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
                    start_coordinate = Some((col, row));
                }
            }
        }

        // for row in matrix.iter() {
        //     println!("{:?}", row);
        // }

        if let Some((col, row)) = start_coordinate {
            // println!("Start coordinate: ({}, {})", col, row);

            // Explore the maze starting from the given coordinate
            explore_maze(&matrix, (col, row))
        } else {
            // println!("No 'S' found in the matrix.");
            (0, 0)
        }
    } else {
        eprintln!("Error opening file: {}", file_path);
        (0, 0)
    }
}
