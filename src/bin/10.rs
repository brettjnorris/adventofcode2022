use std::ops::Range;

#[derive(Debug)]
enum Instruction {
    Add(i32),
    Noop
}

fn parse_instruction(input: &str) -> Instruction {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    match parts[0] {
        "addx" => {
            Instruction::Add(parts[1].parse::<i32>().unwrap())
        },
        _ => Instruction::Noop
    }
}

struct CPU {
    counter: u32,
    register_x: i32,
    delay: u32,
    instructions: Vec<Instruction>,
    next_instruction: Option<Instruction>
}

impl CPU {
    fn from_input(input: &str) -> CPU {
        let instructions = input
            .lines()
            .map(|line| {
                parse_instruction(line)
            })
            .rev()
            .collect::<Vec<Instruction>>();

        CPU {
            instructions,
            next_instruction: None,
            counter: 0,
            register_x: 1,
            delay: 0,
        }
    }

    fn tick(&mut self) {
        self.counter += 1;

        // Apply last instruction
        if self.delay == 0 {
            if let Some(Instruction::Add(val)) =  self.next_instruction {
                 self.register_x += val;
            }

            let instruction = self.instructions.pop();

            if let Some(Instruction::Add(_)) = instruction {
                self.delay = 1;
            }
            self.next_instruction = instruction;
        } else {
            self.delay -= 1;
        }
    }

    fn draw_pixel(&self) -> bool {
        let range = ((self.register_x - 1)..=(self.register_x + 1));

        range.contains(&((self.counter - 1) as i32 % 40))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = CPU::from_input(input);
    let mut signal_strength = 0;

    loop {
        cpu.tick();

        match cpu.counter {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let score = cpu.counter as i32 * cpu.register_x;
                signal_strength += score;
            },
            _ => ()
        }

        if cpu.next_instruction.is_none() {
            break;
        }
    }
    Some(signal_strength as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = CPU::from_input(input);
    let mut output: Vec<&str> = vec![];

    loop {
        cpu.tick();

        if cpu.draw_pixel() {
            output.push("#");
        } else {
            output.push(".");
        }

        if cpu.next_instruction.is_none() {
            break;
        }
    }

    let printed = output
        .chunks(40)
        .map(|chunk| {
            chunk.join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

    Some(printed)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected_output = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n.".to_string();
        assert_eq!(part_two(&input), Some(expected_output));
    }
}
