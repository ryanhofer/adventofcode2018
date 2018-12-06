use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> usize {
    let mut polymer: Vec<char> = INPUT.trim().chars().collect();

    perform_reaction(&mut polymer);

    polymer.len()
}

fn part_two() -> usize {
    let polymer: Vec<char> = INPUT.trim().chars().collect();
    println!("Length before reaction: {}", polymer.len());

    let mut units = HashSet::new();
    for c in &polymer {
        units.insert(c.to_ascii_lowercase());
    }

    let units: String = units.into_iter().collect();
    println!("Units: {}", units);

    let mut best_length = polymer.len();

    for u in units.chars() {
        println!("Reacting polymer with {} removed...", u);

        let mut polymer: Vec<char> = INPUT
            .trim()
            .chars()
            .filter(|c| u != c.to_ascii_lowercase())
            .collect();

        perform_reaction(&mut polymer);

        if polymer.len() < best_length {
            best_length = polymer.len();
        }

        println!("Length after reaction: {}", polymer.len());
    }

    best_length
}

fn perform_reaction(polymer: &mut Vec<char>) {
    loop {
        let mut index = None;

        for (i, pair) in polymer.windows(2).enumerate() {
            let (a, b) = (pair[0], pair[1]);
            if a.is_lowercase() == b.is_lowercase() {
                continue;
            }
            if a.is_lowercase() && a == b.to_ascii_lowercase() {
                index = Some(i);
                break;
            }
            if b.is_lowercase() && b == a.to_ascii_lowercase() {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            polymer.remove(i);
            polymer.remove(i);
        } else {
            break;
        }
    }
}
