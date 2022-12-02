enum MoveType {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

enum Strategy {
    SolveForOutcome,
    SolveForScore,
}

fn parse_move_type(string: &str) -> Option<MoveType> {
    match string {
        "A" | "X" => Some(MoveType::Rock),
        "B" | "Y" => Some(MoveType::Paper),
        "C" | "Z" => Some(MoveType::Scissors),
        _ => None,
    }
}

fn parse_outcome(string: &str) -> Option<Outcome> {
    match string {
        "X" => Some(Outcome::Lose),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

struct Round {
    move_a: Option<MoveType>,
    move_b: Option<MoveType>,
    outcome: Option<Outcome>,
}

impl Round {
    fn from_string(string: &str, strategy: Strategy) -> Round {
        let moves = string.split(" ").collect::<Vec<&str>>();

        match strategy {
            Strategy::SolveForOutcome => Round {
                move_a: Some(parse_move_type(moves[0]).unwrap()),
                move_b: None,
                outcome: Some(parse_outcome(moves[1]).unwrap()),
            },
            Strategy::SolveForScore => Round {
                move_a: Some(parse_move_type(moves[0]).unwrap()),
                move_b: Some(parse_move_type(moves[1]).unwrap()),
                outcome: None,
            },
        }
    }

    fn score(&mut self) -> u32 {
        if self.move_b.is_none() {
            self.find_move()
        }

        let move_score = self.score_move();
        let result_score = self.score_result();

        move_score + result_score
    }

    fn score_move(&self) -> u32 {
        match self.move_b {
            Some(MoveType::Rock) => 1,
            Some(MoveType::Paper) => 2,
            Some(MoveType::Scissors) => 3,
            None => 0,
        }
    }

    fn find_move(&mut self) {
        self.move_b = match (&self.move_a, &self.outcome) {
            (Some(MoveType::Rock), Some(Outcome::Win)) => Some(MoveType::Paper),
            (Some(MoveType::Rock), Some(Outcome::Lose)) => Some(MoveType::Scissors),
            (Some(MoveType::Rock), Some(Outcome::Draw)) => Some(MoveType::Rock),

            (Some(MoveType::Paper), Some(Outcome::Win)) => Some(MoveType::Scissors),
            (Some(MoveType::Paper), Some(Outcome::Lose)) => Some(MoveType::Rock),
            (Some(MoveType::Paper), Some(Outcome::Draw)) => Some(MoveType::Paper),

            (Some(MoveType::Scissors), Some(Outcome::Win)) => Some(MoveType::Rock),
            (Some(MoveType::Scissors), Some(Outcome::Lose)) => Some(MoveType::Paper),
            (Some(MoveType::Scissors), Some(Outcome::Draw)) => Some(MoveType::Scissors),
            (_, _) => None,
        }
    }

    fn score_result(&self) -> u32 {
        match (&self.move_a, &self.move_b) {
            (Some(MoveType::Rock), Some(MoveType::Rock)) => 3,
            (Some(MoveType::Rock), Some(MoveType::Paper)) => 6,
            (Some(MoveType::Rock), Some(MoveType::Scissors)) => 0,

            (Some(MoveType::Paper), Some(MoveType::Rock)) => 0,
            (Some(MoveType::Paper), Some(MoveType::Paper)) => 3,
            (Some(MoveType::Paper), Some(MoveType::Scissors)) => 6,

            (Some(MoveType::Scissors), Some(MoveType::Rock)) => 6,
            (Some(MoveType::Scissors), Some(MoveType::Paper)) => 0,
            (Some(MoveType::Scissors), Some(MoveType::Scissors)) => 3,
            (_, _) => 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let round_scores = input
        .lines()
        .map(|line| Round::from_string(line, Strategy::SolveForScore).score());

    Some(round_scores.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let round_scores = input
        .lines()
        .map(|line| Round::from_string(line, Strategy::SolveForOutcome).score());

    Some(round_scores.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
