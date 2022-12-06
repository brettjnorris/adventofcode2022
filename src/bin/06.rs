struct FrequencyTable {
    table: [u8; 26],
    duplicate_count: u8
}

impl FrequencyTable {
    fn new() -> FrequencyTable {
        FrequencyTable {
            table: [0; 26],
            duplicate_count: 0
        }
    }

    fn increment(&mut self, index: usize) {
        self.table[index] = self.table[index] + 1;

        if self.table[index] == 2 {
            self.duplicate_count = self.duplicate_count + 1;
        }
    }

    fn decrement(&mut self, index: usize) {
        self.table[index] = self.table[index] - 1;

        if self.table[index] == 1 {
            self.duplicate_count = self.duplicate_count - 1;
        }

    }
}

fn find_marker_improved(input: &str, sequence_size: u32) -> Option<u32> {
    let mut frequency_table = FrequencyTable::new();

    let mut index: usize = 0;
    let mut window: Vec<u8> = vec![];

    let input_slice = input.as_bytes();

    loop {
        let char = input_slice[index];

        window.push(char);
        frequency_table.increment((usize::from(char)) % 26);

        if window.len() > sequence_size as usize {
            let first = window[0];
            window = window[1..].to_vec();

            frequency_table.decrement((usize::from(first)) % 26);

            if frequency_table.duplicate_count == 0 {
                break;
            }
        }

        index = index + 1;
    }

    Some(index as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    find_marker_improved(&input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_marker_improved(&input, 14)
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
