use itertools::Itertools;

fn find_marker(input: &str, sequence_size: u32) -> Option<u32> {
    let mut end_index: u32 = 0;
    let mut windows = input
        .as_bytes()
        .windows(sequence_size as usize)
        .enumerate();

    loop {
        let (i, chars) = windows.next().unwrap();

        if chars.into_iter().unique().count() == (sequence_size as usize) {
            end_index = (i as u32) + sequence_size;
            break;
        }
    }

    Some(end_index)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_marker(&input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_marker(&input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
