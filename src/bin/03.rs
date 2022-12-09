use std::collections::HashSet;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

struct Backpack {
    items: HashSet<char>,
    compartments: Vec<HashSet<char>>,
}

impl Backpack {
    fn from_string(input: &str) -> Backpack {
        let (first, second) = input.split_at(input.len() / 2);

        let first_compartment: HashSet<char> = first.chars().into_iter().collect();
        let second_compartment: HashSet<char> = second.chars().into_iter().collect();

        Backpack {
            items: input.chars().into_iter().collect::<HashSet<char>>(),
            compartments: vec![first_compartment, second_compartment],
        }
    }

    fn priority(&self) -> Option<u32> {
        let dupes = self.find_duplicates();
        Some(
            dupes
                .iter()
                .map(|char| score_item(&char).unwrap() as u32)
                .sum(),
        )
    }

    fn find_duplicates(&self) -> Vec<char> {
        let first = &self.compartments[0];
        let second = &self.compartments[1];
        first
            .intersection(second)
            .map(|i| *i)
            .collect::<Vec<char>>()
    }
}

fn find_duplicates(backpacks: Vec<Backpack>) -> Vec<char> {
    let dupes = backpacks.iter().skip(1).fold(
        backpacks[0].items.clone(),
        |acc: HashSet<char>, backpack| {
            acc.intersection(&backpack.items.clone())
                .cloned()
                .collect::<HashSet<char>>()
        },
    );

    Vec::from_iter(dupes)
}

fn score_item(item: &char) -> Option<u8> {
    match ALPHABET.chars().position(|c| &c == item) {
        Some(i) => Some((i + 1) as u8),
        None => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| Backpack::from_string(line).priority())
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let score = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let backpacks = chunk
                .iter()
                .map(|line| Backpack::from_string(line))
                .collect::<Vec<Backpack>>();

            find_duplicates(backpacks)
                .iter()
                .map(|char| score_item(&char).unwrap() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
