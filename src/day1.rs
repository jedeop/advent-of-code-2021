use crate::utils;

fn input() -> Vec<u64> {
    let input = utils::read_input(1);
    input.lines().map(|n| n.parse::<u64>().unwrap()).collect()
}

pub fn part1() -> usize {
    let depths = input();

    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

pub fn part2() -> usize {
    let depths = input();

    let windows: Vec<u64> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect();

    windows
        .iter()
        .zip(windows.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}
