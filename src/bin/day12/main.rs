#[macro_use]
extern crate scan_fmt;

use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");

struct Rule {
    pattern: Vec<bool>,
    result: bool,
}

impl Rule {
    fn from_str(s: &str) -> Self {
        let (pattern, result) = scan_fmt!(s, "{} => {}", String, char);
        let (pattern, result) = (pattern.unwrap(), result.unwrap());
        let pattern: Vec<bool> = pattern.chars().map(|c| c == '#').collect();
        let result = match result {
            '#' => true,
            _ => false,
        };
        Rule { pattern, result }
    }

    fn check(&self, pots: &HashSet<i64>, index: i64) -> bool {
        let offset = self.pattern.len() as i64 / 2;
        let pots_iter = (index - offset..).map(|i| pots.contains(&i));
        self.pattern.iter().zip(pots_iter).all(|(&a, b)| a == b)
    }
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> i64 {
    run_simulation(20)
}

fn part_two() -> i64 {
    run_simulation(50_000_000_000u64)
}

fn run_simulation(num_generations: u64) -> i64 {
    let mut pots = HashSet::new();
    let mut input = INPUT.lines();

    let initial_state = scan_fmt!(input.next().unwrap(), "initial state: {}", String);
    let initial_state = initial_state.unwrap();
    input.next(); // skip blank line

    for (i, c) in initial_state.chars().enumerate() {
        let has_plant = c == '#';
        if has_plant {
            pots.insert(i as i64);
        }
    }

    let mut rules = vec![];
    for line in input {
        // println!("{}", line);
        let rule = Rule::from_str(line);
        rules.push(rule);
    }

    // print!(" 0: ");
    // print_pots(&pots);

    let mut prev_sum: i64 = pots.iter().sum();
    let mut prev_delta: i64 = 0;
    let mut num_generations_with_same_delta: u64 = 0;
    const STABILITY_THRESHOLD: u64 = 100;

    for t in 1..=num_generations {
        let mut new_pots = pots.clone();
        let &min = pots.iter().min().unwrap();
        let &max = pots.iter().max().unwrap();
        for i in min - 2..=max + 2 {
            match rules.iter().find(|rule| rule.check(&pots, i)) {
                Some(rule) => {
                    if rule.result {
                        new_pots.insert(i);
                    } else {
                        new_pots.remove(&i);
                    }
                }
                None => (),
            }
        }
        pots = new_pots;

        let sum: i64 = pots.iter().sum();
        let delta = sum - prev_sum;
        if delta == prev_delta {
            num_generations_with_same_delta += 1;
        } else {
            num_generations_with_same_delta = 0;
        }
        prev_sum = sum;
        prev_delta = delta;

        // print!("{: >2}: ", t);
        // print_pots(&pots);
        // println!("sum={} delta={}", sum, delta);

        if num_generations_with_same_delta >= STABILITY_THRESHOLD {
            let num_generations_remaining = (num_generations - t) as i64;
            let predicted_sum = sum + delta * num_generations_remaining;
            return predicted_sum;
        }
    }

    pots.iter().sum()
}

fn print_pots(pots: &HashSet<i64>) {
    let &min = pots.iter().min().unwrap();
    let &max = pots.iter().max().unwrap();
    let mut s = String::new();
    for i in min..=max {
        let c = match pots.contains(&i) {
            true => '#',
            false => '.',
        };
        s.push(c);
    }
    let sum: i64 = pots.iter().sum();
    println!("{}|{}|{} sum={}", min, s, max, sum);
}
