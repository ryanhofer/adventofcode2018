use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn get_lines() -> std::str::Split<'static, &'static str> {
    INPUT.trim().split("\n")
}

fn main() {
    println!("Part One: {}", one());
    println!("Part Two: {}", two());
}

fn one() -> i64 {
    let mut frequency: i64 = 0;
    for line in get_lines() {
        let delta: i64 = line.trim().parse().unwrap();
        frequency += delta;
    }

    frequency
}

fn two() -> i64 {
    let mut seen = HashSet::new();
    let mut frequency: i64 = 0;
    seen.insert(frequency);

    for line in get_lines().cycle() {
        let delta: i64 = line.trim().parse().unwrap();
        frequency += delta;
        if seen.contains(&frequency) {
            break;
        }
        seen.insert(frequency);
    }

    frequency
}
