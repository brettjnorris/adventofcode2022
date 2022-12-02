pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|val| {
                    match val.parse::<u32>() {
                        Ok(v) => v,
                        Err(e) => 0
                    }
                } )
                .sum()
        }).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|val| {
                    match val.parse::<u32>() {
                        Ok(v) => v,
                        Err(e) => 0
                    }
                } )
                .sum()
        })
        .collect::<Vec<u32>>();

    totals.sort();
    totals.reverse();

    Some(totals[..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
