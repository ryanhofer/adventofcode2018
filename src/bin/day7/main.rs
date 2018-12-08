#[macro_use]
extern crate scan_fmt;

use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &'static str = include_str!("input.txt");
const NUM_WORKERS: usize = 5;

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}

fn part_one() -> String {
    let reqs_per_step = parse_steps(INPUT.trim());

    let mut completed = HashSet::new();
    let mut answer = String::new();

    loop {
        let ready = find_ready(&reqs_per_step, &completed, &vec![]);
        match ready.first() {
            Some(&step) => {
                completed.insert(step);
                answer.push(step);
            }
            None => break,
        }
    }

    answer
}

fn part_two() -> u32 {
    let reqs_per_step = parse_steps(INPUT.trim());

    let mut time = 0;
    let mut jobs = Vec::<Job>::new();
    let mut completed = HashSet::new();

    loop {
        let (completed_jobs, incomplete_jobs): (Vec<Job>, Vec<Job>) =
            jobs.iter().partition(|job| time == job.complete_at);
        jobs = incomplete_jobs;
        for job in completed_jobs {
            completed.insert(job.step);
            println!("Step {} completed at time={}", job.step, time);
        }

        let num_workers_available = NUM_WORKERS - jobs.len();
        if num_workers_available > 0 {
            let ready = find_ready(&reqs_per_step, &completed, &jobs);
            for &step in ready.iter().take(num_workers_available) {
                jobs.push(Job {
                    step,
                    complete_at: time + duration(step),
                });
            }
        }

        if jobs.is_empty() {
            println!("Nothing to do!");
            break;
        }

        time += 1;
    }

    time
}

#[derive(Copy, Clone)]
struct Job {
    step: char,
    complete_at: u32,
}

fn duration(step: char) -> u32 {
    60 + 1 + (step as u32) - ('A' as u32)
}

fn parse_steps(s: &str) -> HashMap<char, Vec<char>> {
    let mut reqs_per_step = HashMap::new();

    for line in s.lines() {
        let (req, step) = scan_fmt!(
            line,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        );
        let (req, step) = (req.unwrap(), step.unwrap());

        reqs_per_step.entry(step).or_insert(vec![]).push(req);
        reqs_per_step.entry(req).or_insert(vec![]);
    }

    reqs_per_step
}

fn find_ready(
    reqs_per_step: &HashMap<char, Vec<char>>,
    completed: &HashSet<char>,
    jobs: &Vec<Job>,
) -> Vec<char> {
    let mut ready: Vec<char> = reqs_per_step
        .iter()
        .filter(|(&step, _)| !completed.contains(&step))
        .filter(|(&step, _)| !jobs.iter().any(|job| job.step == step))
        .filter(|(_, reqs)| reqs.iter().all(|&req| completed.contains(&req)))
        .map(|(&step, _)| step)
        .collect();

    ready.sort();

    ready
}
