use std::collections::HashSet;
use std::ops::Fn;
use std::iter;
use std::str;

const INPUT: &'static str = include_str!("input.txt");

fn get_freq_changes<'a>() -> iter::Map<str::Split<'a, &'a str>, &'static Fn(&str) -> i64> {
    INPUT
        .trim()
        .split("\n")
        .map(&atoi)
}

fn atoi(s: &str) -> i64 {
    s.parse().unwrap_or(0)
}

fn main() {
    println!("Part One: {}", one());
    println!("Part Two: {}", two());
}

fn one() -> i64 {
    let mut frequency: i64 = 0;
    for delta in get_freq_changes() {
        frequency += delta;
    }

    frequency
}

fn two() -> i64 {
    let mut seen = HashSet::new();
    let mut frequency: i64 = 0;
    seen.insert(frequency);

    for delta in get_freq_changes().cycle() {
        frequency += delta;
        if seen.contains(&frequency) {
            break;
        }
        seen.insert(frequency);
    }

    frequency
}
