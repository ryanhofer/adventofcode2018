use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i64 {
    let freq_changes = INPUT.lines().map(|s| s.parse::<i64>().unwrap());
    freq_changes.sum()
}

fn part_two() -> i64 {
    let freq_changes = INPUT.lines().map(|s| s.parse::<i64>().unwrap());
    let mut frequency: i64 = 0;
    let mut seen = HashSet::new();

    for freq_change in freq_changes.cycle() {
        seen.insert(frequency);
        frequency += freq_change;
        if seen.contains(&frequency) {
            break;
        }
    }

    frequency
}
