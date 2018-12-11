use std::collections::HashMap;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> u32 {
    checksum(INPUT.lines())
}

fn checksum<'a, I>(ids: I) -> u32
where
    I: Iterator<Item = &'a str>,
{
    let mut num_containing_double: u32 = 0;
    let mut num_containing_triple: u32 = 0;

    for id in ids {
        let mut letter_counts = HashMap::new();
        for c in id.chars() {
            *letter_counts.entry(c).or_insert(0) += 1;
        }
        if letter_counts.values().any(|&count| count == 2) {
            num_containing_double += 1;
        }
        if letter_counts.values().any(|&count| count == 3) {
            num_containing_triple += 1;
        }
    }

    num_containing_double * num_containing_triple
}

fn part_two() -> String {
    let mut result = String::new();

    'outer: for s1 in INPUT.lines() {
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
