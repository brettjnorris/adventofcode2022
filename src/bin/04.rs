fn contains_range(range_a: Vec<u32>, range_b: Vec<u32>) -> bool {
    range_a[0] >= range_b[0] && range_a[1] <= range_b[1]
}

fn overlaps(range_a: Vec<u32>, range_b: Vec<u32>) -> bool {
    range_a[1] >= range_b[0] && range_a[0] <= range_b[1]
}

fn parse_ranges(line: &str) -> Vec<Vec<u32>> {
    line.split(",")
        .map(|pair| {
            pair.split("-")
                .map(|val| val.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| {
            let ranges = parse_ranges(line);

            contains_range(ranges[0].clone(), ranges[1].clone())
                || contains_range(ranges[1].clone(), ranges[0].clone())
        })
        .count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = input
        .lines()
        .filter(|line| {
            let ranges = parse_ranges(line);

            overlaps(ranges[0].clone(), ranges[1].clone())
                || overlaps(ranges[1].clone(), ranges[0].clone())
        })
        .count();

    Some(count as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
