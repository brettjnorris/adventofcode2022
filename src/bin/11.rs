use num::Integer;
use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    counter: u128,
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    if_true: ActionType,
    if_false: ActionType,
}

impl Monkey {
    fn from_input(input: &str) -> Monkey {
        let lines = input.split("\n").collect::<Vec<&str>>();

        Monkey {
            counter: 0,
            items: Monkey::parse_starting_items(lines[1]),
            operation: Monkey::parse_operation(lines[2]).unwrap(),
            test: Monkey::parse_test(lines[3]),
            if_true: Monkey::parse_action_type(lines[4]),
            if_false: Monkey::parse_action_type(lines[5]),
        }
    }

    fn parse_starting_items(line: &str) -> Vec<u128> {
        let parts: Vec<&str> = line.split("Starting items: ").collect();
        parts[1]
            .split(", ")
            .filter_map(|elem| match elem.parse::<u128>() {
                Ok(val) => Some(val),
                _ => None,
            })
            .collect::<Vec<u128>>()
    }

    fn parse_operation(line: &str) -> Option<Operation> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let operator = parts[parts.len() - 2];
        let operand = parts[parts.len() - 1].parse::<u32>();

        match (operator, operand) {
            ("*", Ok(operand)) => Some(Operation::MultiplyBy(operand)),
            ("*", Err(_)) => Some(Operation::Double),
            ("+", Ok(operand)) => Some(Operation::AddTo(operand)),
            (_, _) => None,
        }
    }

    fn parse_test(line: &str) -> Test {
        let parts: Vec<&str> = line.split("Test: divisible by ").collect();
        let operand = parts[1].parse::<u32>().unwrap();
        Test::DivisibleBy(operand)
    }

    fn parse_action_type(line: &str) -> ActionType {
        let re = Regex::new(r"If (true|false): throw to monkey (\d+)$").unwrap();
        let captures = re.captures(line).unwrap();

        let operand = captures
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<u32>()
            .unwrap();

        ActionType::ThrowTo(operand)
    }

    fn inspect_items(&mut self, divide_worry: bool, lcm: u32) -> Vec<Action> {
        let items = self.items.clone();
        self.items = vec![];
        self.counter += items.len() as u128;

        items
            .into_iter()
            .map(|item| self.inspect_item(item, divide_worry, lcm))
            .collect()
    }

    fn inspect_item(&self, item: u128, divide_worry: bool, lcm: u32) -> Action {
        let mut worry_level: u128 = item;

        match self.operation {
            Operation::AddTo(val) => worry_level += val as u128,
            Operation::Double => worry_level = worry_level * worry_level,
            Operation::MultiplyBy(val) => worry_level = worry_level * (val as u128),
        }

        if divide_worry {
            worry_level = worry_level / 3;
        } else {
            worry_level %= lcm as u128
        }

        let test_result = match self.test {
            Test::DivisibleBy(val) => worry_level % (val as u128) == 0,
        };

        let action_type = match test_result {
            true => self.if_true.clone(),
            false => self.if_false.clone(),
        };

        Action {
            action_type,
            val: worry_level,
        }
    }

    fn add_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

#[derive(Clone, Debug)]
enum Test {
    DivisibleBy(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ActionType {
    ThrowTo(u32),
}

#[derive(Clone, Debug)]
enum Operation {
    AddTo(u32),
    MultiplyBy(u32),
    Double,
}

#[derive(Clone, Debug)]
struct Action {
    action_type: ActionType,
    val: u128,
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .into_iter()
        .map(|chunk| Monkey::from_input(chunk))
        .collect::<Vec<Monkey>>()
}

fn simulate_monkeys(
    monkeys: &mut Vec<Monkey>,
    num_rounds: usize,
    divide_worry: bool,
    lcm: u32,
) -> Option<u128> {
    for _i in 0..num_rounds {
        for j in 0..monkeys.len() {
            let monkey = &mut monkeys[j];
            let actions = monkey.inspect_items(divide_worry, lcm);

            for action in actions {
                let ActionType::ThrowTo(id) = action.action_type;
                let item = action.val.clone();
                let monkey = &mut monkeys[id as usize];

                monkey.add_item(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.counter.cmp(&a.counter));

    monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.counter)
        .reduce(|accum, item| accum * item)
}

fn get_monkey_lcm(monkeys: &Vec<Monkey>) -> u32 {
    monkeys
        .iter()
        .map(|monkey| match monkey.test {
            Test::DivisibleBy(val) => val,
        })
        .reduce(|accum, item| accum.lcm(&item))
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut monkeys = parse_monkeys(input);
    let lcm = get_monkey_lcm(&monkeys);

    simulate_monkeys(&mut monkeys, 20, true, lcm)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut monkeys = parse_monkeys(input);
    let lcm = get_monkey_lcm(&monkeys);

    simulate_monkeys(&mut monkeys, 10000, false, lcm)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }

    #[test]
    fn test_monkey_inspect_case_1() {
        let monkey = Monkey {
            id: 0,
            counter: 0,
            items: vec![79, 78],
            operation: Operation::MultiplyBy(19),
            test: Test::DivisibleBy(23),
            if_true: ActionType::ThrowTo(2),
            if_false: ActionType::ThrowTo(3),
        };

        let action = monkey.inspect_item(79, true, 501);
        assert_eq!(action.action_type, ActionType::ThrowTo(3));
        assert_eq!(action.val, 500);
    }
}
