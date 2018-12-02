use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i64 {
    let mut twos = HashSet::new();
    let mut threes = HashSet::new();
    for s in INPUT.lines() {
        let mut letter_counts = HashMap::new();
        for c in s.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
        if letter_counts.values().any(|&count| count == 2) {
            twos.insert(s);
        }
        if letter_counts.values().any(|&count| count == 3) {
            threes.insert(s);
        }
    }

    (twos.len() * threes.len()) as i64
}

fn part_two() -> String {
    let mut result = String::new();

    'outer:
    for s1 in INPUT.lines() {
        for s2 in INPUT.lines() {
            let mut same = String::new();
            let mut diff_count = 0;
            for (c1, c2) in s1.chars().zip(s2.chars()) {
                if c1 == c2 {
                    same.push(c1);
                } else {
                    diff_count += 1;
                }
            }
            if diff_count == 1 {
                result = same;
                break 'outer;
            }
        }
    }

    result
}
