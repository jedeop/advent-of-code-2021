use crate::utils;

fn input() -> Vec<(String, u64)> {
    let input = utils::read_input(2);
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            (
                line.next().unwrap().to_owned(),
                line.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

pub fn part1() -> u64 {
    let commands = input();
    let mut horizontal = 0;
    let mut depth = 0;

    for (cmd, value) in commands {
        match &cmd[..] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => unreachable!(),
        }
    }

    horizontal * depth
}

pub fn part2() -> u64 {
    let commands = input();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (cmd, value) in commands {
        match &cmd[..] {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => unreachable!(),
        }
    }

    horizontal * depth
}
