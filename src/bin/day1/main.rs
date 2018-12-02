use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i64 {
    let mut frequency: i64 = 0;
    for delta in INPUT.lines().map(atoi) {
        frequency += delta;
    }

    frequency
}

fn part_two() -> i64 {
    let mut seen = HashSet::new();
    let mut frequency: i64 = 0;
    seen.insert(frequency);

    for delta in INPUT.lines().map(atoi).cycle() {
        frequency += delta;
        if seen.contains(&frequency) {
            break;
        }
        seen.insert(frequency);
    }

    frequency
}

fn atoi(s: &str) -> i64 {
    s.parse().unwrap_or(0)
}
