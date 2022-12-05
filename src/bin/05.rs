struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn from_input(mut input: Vec<&str>) -> State {
        input.reverse();

        let mut stacks = State::initialize_stacks(input.clone());

        input.iter().skip(1).for_each(|&line| {
            for i in 0..stacks.len() {
                let index = 1 + (i * 4);
                let item = line.chars().nth(index);

                if let Some(val) = item {
                    if val.to_string() != " " {
                        stacks[i].push(val);
                    }
                }
            }
        });

        State { stacks }
    }

    fn initialize_stacks(input: Vec<&str>) -> Vec<Vec<char>> {
        let mut stacks: Vec<Vec<char>> = vec![];
        let num_stacks = input[0].split_whitespace().collect::<Vec<&str>>().len();

        for _ in 0..num_stacks {
            stacks.push(vec![])
        }

        stacks
    }

    fn apply_move_singular(&mut self, move_instruction: &Instruction) {
        for _ in 0..move_instruction.quantity {
            let stack_from = self
                .stacks
                .get_mut((move_instruction.from - 1) as usize)
                .unwrap();

            if let Some(item) = stack_from.pop() {
                let stack_to = self
                    .stacks
                    .get_mut((move_instruction.to - 1) as usize)
                    .unwrap();
                stack_to.push(item);
            }
        }
    }

    fn apply_move_grouped(&mut self, move_instruction: &Instruction) {
        let stack_from = self
            .stacks
            .get_mut((move_instruction.from - 1) as usize)
            .unwrap();
        let mut items = vec![];

        for _ in 0..move_instruction.quantity {
            if let Some(item) = stack_from.pop() {
                items.push(item);
            }
        }

        let stack_to = self
            .stacks
            .get_mut((move_instruction.to - 1) as usize)
            .unwrap();
        let reversed = items.iter().rev().collect::<Vec<&char>>();
        for item in reversed {
            stack_to.push(*item);
        }
    }

    fn get_top_stacks(&self) -> String {
        let mut signature: Vec<char> = vec![];
        self.stacks.clone().into_iter().for_each(|stack| {
            if let Some(val) = stack.last() {
                signature.push(val.clone());
            }
        });

        signature.iter().collect::<String>()
    }
}

struct Instruction {
    quantity: u8,
    from: u8,
    to: u8,
}

impl Instruction {
    fn from_block(input: &str) -> Vec<Instruction> {
        let lines = input.lines().collect::<Vec<&str>>();

        lines
            .iter()
            .cloned()
            .rev()
            .take_while(|line| !line.is_empty())
            .map(|line| Instruction::from_string(line))
            .collect::<Vec<Instruction>>()
            .into_iter()
            .rev()
            .collect()
    }

    fn from_string(input: &str) -> Instruction {
        let parts = input.split(" ").collect::<Vec<&str>>();

        let quantity: u8 = parts[1].parse::<u8>().unwrap();
        let from: u8 = parts[3].parse::<u8>().unwrap();
        let to: u8 = parts[5].parse::<u8>().unwrap();

        Instruction { quantity, from, to }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let stack_state: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();

    let mut state = State::from_input(stack_state);
    let instructions = Instruction::from_block(input.clone());

    instructions
        .iter()
        .for_each(|instruction| state.apply_move_singular(instruction));

    Some(state.get_top_stacks())
}

pub fn part_two(input: &str) -> Option<String> {
    let stack_state: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();

    let mut state = State::from_input(stack_state);
    let instructions = Instruction::from_block(input.clone());

    instructions
        .iter()
        .for_each(|instruction| state.apply_move_grouped(instruction));

    Some(state.get_top_stacks())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }

    #[test]
    fn test_instruction_from_string() {
        let input = "move 3 from 1 to 3";
        let instruction = Instruction::from_string(input);
        assert_eq!(instruction.quantity, 3);
        assert_eq!(instruction.from, 1);
        assert_eq!(instruction.to, 3);
    }
}
