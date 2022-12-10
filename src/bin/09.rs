use itertools::Itertools;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i32, i32);

impl Point {
    fn to_direction(&self, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Up => Some(Point(self.0 + 1, self.1)),
            Direction::Down => Some(Point(self.0 - 1, self.1)),
            Direction::Left => Some(Point(self.0, self.1 - 1)),
            Direction::Right => Some(Point(self.0, self.1 + 1)),
            Direction::UpRight => Some(Point(self.0 + 1, self.1 + 1)),
            Direction::UpLeft => Some(Point(self.0 + 1, self.1 - 1)),
            Direction::DownRight => Some(Point(self.0 - 1, self.1 + 1)),
            Direction::DownLeft => Some(Point(self.0 - 1, self.1 - 1)),
            Direction::Unknown => None,
        }
    }

    fn distance_to(&self, point: &Point) -> (i32, i32) {
        (point.0 - self.0, point.1 - self.1)
    }

    fn direction_to(&self, point: &Point) -> Option<Direction> {
        let distance_to = self.distance_to(point);

        match distance_to {
            (0, 2) => Some(Direction::Right),
            (-2, 0) => Some(Direction::Down),
            (0, -2) => Some(Direction::Left),
            (2, 0) => Some(Direction::Up),
            (1, 2) | (2, 1) | (2, 2) => Some(Direction::UpRight),
            (1, -2) | (2, -1) | (2, -2) => Some(Direction::UpLeft),
            (-1, -2) | (-2, -1) | (-2, -2) => Some(Direction::DownLeft),
            (-1, 2) | (-2, 1) | (-2, 2) => Some(Direction::DownRight),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Unknown,
}

impl Direction {
    fn from_string(input: &str) -> Direction {
        match input {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "D" => Direction::Down,
            _ => Direction::Unknown,
        }
    }
}

struct Board {
    knots: Vec<Knot>,
}

impl Board {
    fn new(knot_count: usize) -> Board {
        Board {
            knots: vec![Knot::new(); knot_count],
        }
    }

    fn process_move(&mut self, input: &str) {
        let parts = input.split_whitespace().collect::<Vec<&str>>();
        let direction = Direction::from_string(parts[0]);
        let steps: u32 = parts[1].parse::<u32>().unwrap();

        for _i in 0..steps {
            let mut last_point = Point(0, 0);

            for (j, knot) in self.knots.iter_mut().enumerate() {
                if j == 0 {
                    let new_point = knot.apply_move(&direction);
                    last_point = new_point;
                } else {
                    if let Some(follow_direction) = knot.current_position.direction_to(&last_point)
                    {
                        let new_point = knot.apply_move(&follow_direction);
                        last_point = new_point;
                    } else {
                        last_point = knot.current_position.clone();
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Knot {
    current_position: Point,
    past_positions: Vec<Point>,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            current_position: Point(0, 0),
            past_positions: vec![Point(0, 0)],
        }
    }

    fn apply_move(&mut self, direction: &Direction) -> Point {
        if let Some(new_position) = self.current_position.to_direction(direction) {
            self.current_position = new_position.clone();
            self.past_positions.push(new_position.clone());
        } else {
            println!("no move");
        }

        self.current_position.clone()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board::new(2);

    input.lines().for_each(|line| board.process_move(line));
    let unique_positions = board.knots.last().unwrap().past_positions.iter().unique();

    Some(unique_positions.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Board::new(10);

    input.lines().for_each(|line| board.process_move(line));
    let unique_positions = board.knots[9].past_positions.iter().unique();

    Some(unique_positions.count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }

    #[test]
    fn test_point_to_distance() {
        let point_a = Point(0, 0);
        let point_b = Point(1, 0);

        assert_eq!(point_a.distance_to(&point_b), (1, 0))
    }
}
